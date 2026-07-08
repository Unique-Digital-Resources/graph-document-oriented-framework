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
    
    rootElement.innerHTML = '';
    renderNode(rootNode, rootElement);
  }

  function renderNode(nodeData, parentEl) {
    let el = document.createElement(nodeData.tag);
    el.id = nodeData.id;
    
    // Apply text content if any
    if (nodeData.text) {
      el.innerText = nodeData.text;
    }
    
    // Apply layout bounds ONLY if they are greater than 0
    // This allows CSS to size elements (like inputs) that don't have explicit bounds
    if (nodeData.bounds && nodeData.bounds[2] > 0 && nodeData.bounds[3] > 0) {
      el.style.position = 'absolute';
      el.style.left = nodeData.bounds[0] + 'px';
      el.style.top = nodeData.bounds[1] + 'px';
      el.style.width = nodeData.bounds[2] + 'px';
      el.style.height = nodeData.bounds[3] + 'px';
      el.style.boxSizing = 'border-box';
    }

    // Apply CSS classes for inspector layout
    if (nodeData.tag === 'div' && nodeData.children.length > 0) {
      el.className = 'gdf-inspector';
    }
    
    // If it's a custom element, call update
    if (nodeData.tag.includes('-')) {
      el.update(nodeData);
    }
    
    // Recursively render children
    if (nodeData.children) {
      nodeData.children.forEach(childData => {
        renderNode(childData, el);
      });
    }
    
    parentEl.appendChild(el);
  }

  window.addEventListener('gdf-input', (e) => {
    wasm.handle_dom_event(JSON.stringify(e.detail));
    renderUI(); 
  });

  renderUI();
}

run();