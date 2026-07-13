export class Vector3Input extends HTMLElement {
  update(nodeData) {
    this._data = nodeData;
    let val = nodeData.props.value.v; 
    let x = val[0].v;
    let y = val[1].v;
    let z = val[2].v;

    if (!this.querySelector('.x')) {
      this.innerHTML = `
        <div class="gdf-vec3-container">
          <span class="gdf-label">${nodeData.props.property}</span>
          <div class="gdf-vec3-inputs">
            <input type="number" aria-label="${nodeData.props.property} X" class="x" value="${x}" step="0.1">
            <input type="number" aria-label="${nodeData.props.property} Y" class="y" value="${y}" step="0.1">
            <input type="number" aria-label="${nodeData.props.property} Z" class="z" value="${z}" step="0.1">
          </div>
        </div>
      `;

      this.querySelectorAll('input').forEach(input => {
        // Fires continuously while typing or holding arrows
        input.addEventListener('input', () => {
          let x = parseFloat(this.querySelector('.x').value);
          let y = parseFloat(this.querySelector('.y').value);
          let z = parseFloat(this.querySelector('.z').value);
          
          window.dispatchEvent(new CustomEvent('gdf-input', {
            detail: {
              type: 'custom',
              target: this.id,
              command_id: 'SetMeshTransform',
              params: {
                mesh_id: this._data.props.target_node,
                property: this._data.props.property,
                value: [x, y, z]
              }
            }
          }));
        });

        // Fires when you press Enter or click away
        input.addEventListener('change', () => {
          let x = parseFloat(this.querySelector('.x').value);
          let y = parseFloat(this.querySelector('.y').value);
          let z = parseFloat(this.querySelector('.z').value);
          
          window.dispatchEvent(new CustomEvent('gdf-change', {
            detail: {
              type: 'custom',
              target: this.id,
              command_id: 'SetMeshTransform',
              params: {
                mesh_id: this._data.props.target_node,
                property: this._data.props.property,
                value: [x, y, z]
              }
            }
          }));
        });
      });
    } else {
      // Update values directly only if the user isn't actively typing in them
      let xInput = this.querySelector('.x');
      let yInput = this.querySelector('.y');
      let zInput = this.querySelector('.z');

      if (document.activeElement !== xInput) xInput.value = x;
      if (document.activeElement !== yInput) yInput.value = y;
      if (document.activeElement !== zInput) zInput.value = z;
    }
  }
}
customElements.define('vector3-input', Vector3Input);