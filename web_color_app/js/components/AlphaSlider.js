import { RangeSlider } from './RangeSlider.js';

export class AlphaSlider extends RangeSlider {
  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || {};
    const colorArray = Array.isArray(props.value) ? props.value : [0, 100, 50, 1];
    const index = props.index !== undefined ? props.index : 3;
    const val = colorArray[index] !== undefined ? colorArray[index] : 1;

    if (!this.querySelector('input')) {
      this.className = 'range-group';
      this.innerHTML = `
        <span class="slider-label">A</span>
        <div class="alpha-wrapper">
          <div class="alpha-gradient"></div>
          <input type="range" min="0" max="1" step="0.01" value="${val}">
        </div>
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
    const grad = this.querySelector('.alpha-gradient');
    
    if (document.activeElement !== input) input.value = val;
    if (valSpan) valSpan.textContent = parseFloat(val).toFixed(2);
    
    const hue = colorArray[0] || 0, sat = colorArray[1] || 100, light = colorArray[2] || 50;
    const rgb = this.hslToRgb(hue, sat, light);
    grad.style.background = `linear-gradient(to right, rgba(${rgb[0]},${rgb[1]},${rgb[2]}, 0), rgba(${rgb[0]},${rgb[1]},${rgb[2]}, 1))`;
  }

  hslToRgb(h, s, l) {
    h /= 360; s /= 100; l /= 100;
    let r, g, b;
    if (s === 0) { r = g = b = l; } else {
      const hue2rgb = (p, q, t) => {
        if (t < 0) t += 1; if (t > 1) t -= 1;
        if (t < 1/6) return p + (q - p) * 6 * t;
        if (t < 1/2) return q;
        if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
        return p;
      };
      const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
      const p = 2 * l - q;
      r = hue2rgb(p, q, h + 1/3); g = hue2rgb(p, q, h); b = hue2rgb(p, q, h - 1/3);
    }
    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
  }
}
customElements.define('alpha-slider', AlphaSlider);