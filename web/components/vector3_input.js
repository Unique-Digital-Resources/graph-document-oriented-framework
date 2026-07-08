export class Vector3Input extends HTMLElement {
  update(nodeData) {
    let val = nodeData.props.value.v; 
    let x = val[0].v;
    let y = val[1].v;
    let z = val[2].v;

    this.innerHTML = `
      <div class="gdf-vec3-container">
        <span class="gdf-label">${nodeData.props.property}</span>
        <div class="gdf-vec3-inputs">
          <input type="number" aria-label="${nodeData.props.property} X" id="${nodeData.id}-x" name="${nodeData.id}-x" class="x" value="${x}" step="0.1">
          <input type="number" aria-label="${nodeData.props.property} Y" id="${nodeData.id}-y" name="${nodeData.id}-y" class="y" value="${y}" step="0.1">
          <input type="number" aria-label="${nodeData.props.property} Z" id="${nodeData.id}-z" name="${nodeData.id}-z" class="z" value="${z}" step="0.1">
        </div>
      </div>
    `;
    this.querySelectorAll('input').forEach(input => {
      input.addEventListener('change', () => {
        let x = parseFloat(this.querySelector('.x').value);
        let y = parseFloat(this.querySelector('.y').value);
        let z = parseFloat(this.querySelector('.z').value);
        
        window.dispatchEvent(new CustomEvent('gdf-input', {
          detail: {
            type: 'custom',
            target: this.id,
            command_id: 'SetMeshTransform',
            params: {
              mesh_id: nodeData.props.target_node,
              property: nodeData.props.property,
              value: [x, y, z]
            }
          }
        }));
      });
    });
  }
}
customElements.define('vector3-input', Vector3Input);