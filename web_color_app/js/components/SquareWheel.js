import { ColorWheel } from './ColorWheel.js';

export class SquareWheel extends ColorWheel {
  updateBackground(color, size) {
    if (!this.hueRing) {
      this.containerEl.innerHTML = `
        <div class="cw-hue-ring"></div>
        <div class="cw-hue-mask"></div>
        <div class="cw-square-layer"></div>
        <div class="cw-thumb cw-hue-thumb"></div>
        <div class="cw-thumb cw-picker-thumb"></div>
      `;
      this.hueRing = this.querySelector('.cw-hue-ring');
      this.hueMask = this.querySelector('.cw-hue-mask');
      this.squareDiv = this.querySelector('.cw-square-layer');
      this.hueThumb = this.querySelector('.cw-hue-thumb');
      this.pickerThumb = this.querySelector('.cw-picker-thumb');
    }

    const center = this.getCenter(size);
    const outerRadius = this.getOuterRadius(size);
    const innerRadius = this.getInnerRadius(size);
    const halfSize = innerRadius * 0.7;

    // FIX: Always update sizes to handle dynamic resizing
    this.hueRing.style.width = `${outerRadius * 2}px`;
    this.hueRing.style.height = `${outerRadius * 2}px`;
    this.hueRing.style.left = `${center - outerRadius}px`;
    this.hueRing.style.top = `${center - outerRadius}px`;

    this.hueMask.style.width = `${innerRadius * 2}px`;
    this.hueMask.style.height = `${innerRadius * 2}px`;
    this.hueMask.style.left = `${center - innerRadius}px`;
    this.hueMask.style.top = `${center - innerRadius}px`;

    this.squareDiv.style.width = `${halfSize * 2}px`;
    this.squareDiv.style.height = `${halfSize * 2}px`;
    this.squareDiv.style.left = `${center - halfSize}px`;
    this.squareDiv.style.top = `${center - halfSize}px`;
    
    const pureHue = `hsl(${color[0]}, 100%, 50%)`;
    this.squareDiv.style.background = `linear-gradient(to top, black, transparent), linear-gradient(to right, white, ${pureHue})`;
  }

  updateThumbs(color, size) {
    const center = this.getCenter(size);
    const outerRadius = this.getOuterRadius(size);
    const innerRadius = this.getInnerRadius(size);
    const midR = (outerRadius + innerRadius) / 2.0;

    const angleRad = (color[0] - 90.0) * Math.PI / 180.0;
    const hx = center + midR * Math.cos(angleRad);
    const hy = center + midR * Math.sin(angleRad);
    this.hueThumb.style.left = `${hx}px`;
    this.hueThumb.style.top = `${hy}px`;
    this.hueThumb.style.backgroundColor = `hsl(${color[0]}, 100%, 50%)`;

    const half = innerRadius * 0.7;
    const s = color[1] / 100;
    const l = color[2] / 100;
    const px = center - half + s * (half * 2);
    const effL = l / (1 - 0.5 * s);
    const py = center - half + (1 - effL) * (half * 2);
    this.pickerThumb.style.left = `${px}px`;
    this.pickerThumb.style.top = `${py}px`;
    this.pickerThumb.style.backgroundColor = `hsla(${color[0]}, ${color[1]}%, ${color[2]}%, ${color[3]})`;
  }

  getColorFromCoords(x, y, size, mode) {
    const center = this.getCenter(size);
    const innerRadius = this.getInnerRadius(size);
    const currentColor = this._color;

    if (mode === 'hue') {
      const angle = Math.atan2(y - center, x - center) * 180 / Math.PI + 90;
      return [(angle + 360) % 360, currentColor[1], currentColor[2], currentColor[3]];
    } else {
      const halfSize = innerRadius * 0.7;
      const minX = center - halfSize, minY = center - halfSize, sz = halfSize * 2;
      const px = Math.max(0, Math.min(1, (x - minX) / sz));
      const py = Math.max(0, Math.min(1, (y - minY) / sz));
      const s = px * 100;
      const baseL = (1 - py) * 100;
      const l = baseL * (1 - 0.5 * px);
      return [currentColor[0], s, l, currentColor[3]];
    }
  }
}
customElements.define('square-wheel', SquareWheel);