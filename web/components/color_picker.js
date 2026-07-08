export class ColorPicker extends HTMLElement {
  update(nodeData) {
    let val = nodeData.props.value.v;
    let r = val[0].v;
    let g = val[1].v;
    let b = val[2].v;

    let color = new THREE.Color(r, g, b).getHexString();
    this.innerHTML = `
      <div class="gdf-color-container">
        <label for="${nodeData.id}-color" class="gdf-label">Face Color</label>
        <input type="color" aria-label="Face Color" id="${nodeData.id}-color" name="${nodeData.id}-color" value="#${color}">
      </div>
    `;
    this.querySelector('input').addEventListener('change', (e) => {
      let c = new THREE.Color(e.target.value);
      window.dispatchEvent(new CustomEvent('gdf-input', {
        detail: {
          type: 'custom',
          target: this.id,
          command_id: 'SetFaceColor',
          params: {
            face_id: nodeData.props.target_node,
            value: [c.r, c.g, c.b]
          }
        }
      }));
    });
  }
}
customElements.define('color-picker', ColorPicker);