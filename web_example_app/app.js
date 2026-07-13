import { GdfNode } from './components/gdf_node.js';
import { Viewport3D } from './components/viewport_3d.js';
import { Vector3Input } from './components/vector3_input.js';
import { ColorPicker } from './components/color_picker.js';

async function run() {
  const wasm = await import('./pkg/example_app.js');
  await wasm.default();
  window.wasm = wasm;
  wasm.init_app();

  const rootElement = document.getElementById('gdf-root');

  function renderUI() {
    const uiJson = wasm.get_ui_state();
    const rootNode = JSON.parse(uiJson);
    let components = rootNode.children || [];

    // Sort components deterministically so HashSet randomness doesn't affect DOM order
    const orderMap = ["position", "rotation", "scale"];
    components.sort((a, b) => {
      if (a.tag === 'viewport-3d') return -1;
      if (b.tag === 'viewport-3d') return 1;
      
      if (a.tag === 'vector3-input' && b.tag === 'vector3-input') {
        let pA = a.props.property || "";
        let pB = b.props.property || "";
        return orderMap.indexOf(pA) - orderMap.indexOf(pB);
      }
      
      if (a.tag === 'color-picker') return 1;
      if (b.tag === 'color-picker') return -1;
      
      return 0;
    });

    // Create Static HTML Layout (Pure CSS control)
    if (!document.getElementById('gdf-layout')) {
      rootElement.innerHTML = `
        <div id="gdf-layout" class="gdf-layout-row">
          <div id="gdf-viewport-container" class="gdf-viewport-container"></div>
          <div id="gdf-inspector" class="gdf-inspector"></div>
        </div>
      `;
    }

    let viewportContainer = document.getElementById('gdf-viewport-container');
    let inspectorContainer = document.getElementById('gdf-inspector');

    let lastInspectorNode = null;
    let lastViewportNode = null;

    // Slot components into the static layout
    components.forEach(nodeData => {
      let el = document.getElementById(nodeData.id);
      
      if (!el) {
        el = document.createElement(nodeData.tag);
        el.id = nodeData.id;
      }

      let targetContainer = (nodeData.tag === 'viewport-3d') ? viewportContainer : inspectorContainer;
      let lastNode = (nodeData.tag === 'viewport-3d') ? lastViewportNode : lastInspectorNode;

      // 1. Ensure it's in the correct container
      if (el.parentElement !== targetContainer) {
        targetContainer.appendChild(el);
      } else {
        // 2. Ensure it's in the correct order WITHOUT moving it if it's already correct
        let expectedNextSibling = lastNode ? lastNode.nextSibling : targetContainer.firstChild;
        if (el !== expectedNextSibling) {
          // Only move if it's out of place
          targetContainer.insertBefore(el, expectedNextSibling);
        }
      }

      if (nodeData.tag === 'viewport-3d') {
        lastViewportNode = el;
      } else {
        lastInspectorNode = el;
      }

      // Update the component
      if (el.update) {
        el.update(nodeData);
      }
    });
  }

  // Real-time updates (typing, dragging) - NO DOM rebuild!
  window.addEventListener('gdf-input', (e) => {
    wasm.handle_dom_event(JSON.stringify(e.detail));
    let vp = document.querySelector('viewport-3d');
    if (vp && vp.render3D) {
      vp.render3D();
    }
  });

  // Final sync (focus lost, mouse released) - Rebuild DOM safely
  window.addEventListener('gdf-change', (e) => {
    wasm.handle_dom_event(JSON.stringify(e.detail));
    renderUI(); 
  });

  renderUI();
}

run();