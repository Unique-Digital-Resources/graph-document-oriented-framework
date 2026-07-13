import './js/components/RangeSlider.js';
import './js/components/HueSlider.js';
import './js/components/SatSlider.js';
import './js/components/LightSlider.js';
import './js/components/AlphaSlider.js';
import './js/components/ColorWheel.js';
import './js/components/CircleWheel.js';
import './js/components/SquareWheel.js';
import './js/components/TriangleWheel.js';
import './js/components/HarmonyWheel.js';
import './js/components/ColorPreview.js';
import './js/components/PaletteGrid.js';
import './js/components/RectTest.js';
import './js/components/WheelMode.js';

async function run() {
  const wasm = await import('./pkg/color_app.js');
  await wasm.default();
  window.wasm = wasm;
  wasm.init_app();

  const rootElement = document.getElementById('gdf-root');

  function flattenTree(node) {
    let flat = [];
    if (node.children) {
      node.children.forEach(child => {
        flat.push(child);
        flat = flat.concat(flattenTree(child));
      });
    }
    return flat;
  }

  function renderUI() {
    const state = JSON.parse(wasm.get_ui_state());
    let components = flattenTree(state);

    const order = ['rect-test', 'wheel-mode', 'circle-wheel', 'square-wheel', 'triangle-wheel', 'ranges-wheel', 'hue-slider', 'sat-slider', 'light-slider', 'alpha-slider', 'color-preview', 'harmony-wheel', 'palette-grid'];
    components.sort((a, b) => {
      const aKind = a.kind || a.tag;
      const bKind = b.kind || b.tag;
      return order.indexOf(aKind) - order.indexOf(bKind);
    });

    if (!document.getElementById('app-layout')) {
      rootElement.innerHTML = `
        <div id="app-layout" style="display:flex; width:100vw; height:100vh;">
          <div id="rect-panel">
            <rect-test id="rect-test-ui"></rect-test>
          </div>
          <div id="card-panel">
            <div class="color-card">
              <div class="slot" data-slot="wheel-mode"></div>
              <div class="visual-wrapper">
                <div class="slot" data-slot="wheel"></div>
              </div>
              <div class="ranges-area slot" data-slot="sliders"></div>
              <div class="slot" data-slot="color-preview"></div>
              <palette-grid data-slot="palette-grid"></palette-grid> 
            </div>
          </div>
        </div>
      `;
    }

    // Determine current wheel mode to manage visibility
    const modeNode = components.find(c => (c.kind || c.tag) === 'wheel-mode');
    const currentMode = modeNode?.props?.mode || 'Ranges';
    const visualWrapper = document.querySelector('.visual-wrapper');
    if (visualWrapper) {
      visualWrapper.style.display = currentMode === 'Ranges' ? 'none' : 'block';
    }

    components.forEach(nodeData => {
      const tag = nodeData.kind || nodeData.tag;
      if (!tag) return;

      let el = document.getElementById(nodeData.id);
      
      // If the tag changed (e.g. circle-wheel -> square-wheel), destroy old element
      if (el && el.tagName.toLowerCase() !== tag) {
        el.remove();
        el = null;
      }

      if (!el) {
        el = document.createElement(tag);
        el.id = nodeData.id;
      }

      let targetContainer = null;
      let shouldDisplay = true;

      if (tag === 'rect-test') targetContainer = document.getElementById('rect-panel');
      else if (tag === 'wheel-mode') targetContainer = document.querySelector('[data-slot="wheel-mode"]');
      else if (tag.endsWith('-wheel')) {
        targetContainer = document.querySelector('[data-slot="wheel"]');
        shouldDisplay = currentMode !== 'Ranges'; // Hide wheel in Ranges mode
      }
      else if (tag.endsWith('-slider')) {
        targetContainer = document.querySelector('[data-slot="sliders"]');
        // Original visibility logic
        if (currentMode === 'Ranges') shouldDisplay = true;
        else if (currentMode === 'Circle') shouldDisplay = (tag === 'light-slider' || tag === 'alpha-slider');
        else shouldDisplay = (tag === 'alpha-slider');
      }
      else if (tag === 'color-preview') targetContainer = document.querySelector('[data-slot="color-preview"]');
      else if (tag === 'palette-grid') targetContainer = document.querySelector('[data-slot="palette-grid"]');

      if (targetContainer) {
        if (el.parentElement !== targetContainer) targetContainer.appendChild(el);
        el.style.display = shouldDisplay ? 'flex' : 'none';
        if (el.update) el.update(nodeData);
      }
    });
  }

  window.addEventListener('gdf-input', (e) => {
    wasm.handle_dom_event(JSON.stringify(e.detail));
    renderUI(); 
  });

  window.addEventListener('gdf-change', (e) => {
    wasm.handle_dom_event(JSON.stringify(e.detail));
    renderUI(); 
  });

  renderUI();
}

run();