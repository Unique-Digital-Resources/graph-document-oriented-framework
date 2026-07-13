import { RangeSlider } from './RangeSlider.js';

export class SatSlider extends RangeSlider {
  updateVisuals(colorArray, label, input) {
    input.min = 0;
    input.max = 100;
    input.step = 1;
    
    if (label) label.textContent = 'S';
    
    const hue = colorArray[0] !== undefined ? colorArray[0] : 0;
    const light = colorArray[2] !== undefined ? colorArray[2] : 50;
    input.style.background = `linear-gradient(to right, hsl(${hue}, 0%, ${light}%), hsl(${hue}, 100%, ${light}%))`;
  }
}

customElements.define('sat-slider', SatSlider);