export class RangeSlider extends HTMLElement {
  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || {};
    const colorArray = Array.isArray(props.value) ? props.value : [0, 100, 50, 1];
    const index = props.index !== undefined ? props.index : 0;
    const val = colorArray[index] !== undefined ? colorArray[index] : 0;

    if (!this.querySelector('input')) {
      this.className = 'range-group';
      this.innerHTML = `
        <span class="slider-label"></span>
        <input type="range" min="0" max="100" step="0.1" value="${val}">
        <span class="slider-val"></span>
      `;
      
      const input = this.querySelector('input');
      const valSpan = this.querySelector('.slider-val');

      input.addEventListener('input', () => {
        window.dispatchEvent(new CustomEvent('gdf-input', { detail: {
          type: 'custom', target: this.id, command_id: 'SetDirectColorProp',
          params: { target_node: props.target_node, prop: props.prop, index: index, value: parseFloat(input.value) }
        }}));
      });

      input.addEventListener('change', () => {
        window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
          type: 'custom', target: this.id, command_id: 'SetDirectColorProp',
          params: { target_node: props.target_node, prop: props.prop, index: index, value: parseFloat(input.value) }
        }}));
      });
    }
    
    const input = this.querySelector('input');
    const valSpan = this.querySelector('.slider-val');
    const label = this.querySelector('.slider-label');
    
    if (document.activeElement !== input) input.value = val;
    if (valSpan) valSpan.textContent = parseFloat(val).toFixed(2);
    
    this.updateVisuals(colorArray, label, input);
  }

  updateVisuals(colorArray, label, input) {}
}
customElements.define('range-slider', RangeSlider);