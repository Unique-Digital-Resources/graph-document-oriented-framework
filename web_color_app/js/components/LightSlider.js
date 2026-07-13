import { RangeSlider } from './RangeSlider.js';

export class LightSlider extends RangeSlider {
  updateVisuals(colorArray, label, input) {
    input.min = 0;
    input.max = 100;
    input.step = 1;
    
    if (label) label.textContent = 'L';
    
    const hue = colorArray[0] !== undefined ? colorArray[0] : 0;
    const sat = colorArray[1] !== undefined ? colorArray[1] : 100;
    input.style.background = `linear-gradient(to right, #000, hsl(${hue}, ${sat}%, 50%), #fff)`;
  }
}

customElements.define('light-slider', LightSlider);