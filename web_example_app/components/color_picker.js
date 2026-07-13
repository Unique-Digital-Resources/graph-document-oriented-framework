export class ColorPicker extends HTMLElement {
  update(nodeData) {
    this._data = nodeData;
    let val = nodeData.props.value.v;
    let r = val[0].v;
    let g = val[1].v;
    let b = val[2].v;

    let color = new THREE.Color(r, g, b).getHexString();

    if (!this.querySelector('input')) {
      this.innerHTML = `
        <div class="gdf-color-container">
          <label for="${nodeData.id}-color" class="gdf-label">Face Color</label>
          <input type="color" aria-label="Face Color" id="${nodeData.id}-color" value="#${color}">
        </div>
      `;

      let input = this.querySelector('input');
      
      // Fires continuously while dragging the color picker
      input.addEventListener('input', (e) => {
        let c = new THREE.Color(e.target.value);
        window.dispatchEvent(new CustomEvent('gdf-input', {
          detail: {
            type: 'custom',
            target: this.id,
            command_id: 'SetFaceColor',
            params: {
              mesh_id: this._data.props.target_node,
              face_index: this._data.props.face_index,
              value: [c.r, c.g, c.b]
            }
          }
        }));
      });

      // Fires when the color picker popup closes
      input.addEventListener('change', (e) => {
        let c = new THREE.Color(e.target.value);
        window.dispatchEvent(new CustomEvent('gdf-change', {
          detail: {
            type: 'custom',
            target: this.id,
            command_id: 'SetFaceColor',
            params: {
              mesh_id: this._data.props.target_node,
              face_index: this._data.props.face_index,
              value: [c.r, c.g, c.b]
            }
          }
        }));
      });
    } else {
      let input = this.querySelector('input');
      if (document.activeElement !== input) {
        input.value = `#${color}`;
      }
    }
  }
}
customElements.define('color-picker', ColorPicker);