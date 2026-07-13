import { RangeSlider } from './RangeSlider.js';

export class HueSlider extends RangeSlider {
  updateVisuals(colorArray, label, input) {
    input.min = 0;
    input.max = 360;
    input.step = 1;
    
    if (label) label.textContent = 'H';
    
    // Hue background is static
    input.style.background = `linear-gradient(to right, #f00 0%, #ff0 17%, #0f0 33%, #0ff 50%, #00f 67%, #f0f 83%, #f00 100%)`;
  }
}

customElements.define('hue-slider', HueSlider);