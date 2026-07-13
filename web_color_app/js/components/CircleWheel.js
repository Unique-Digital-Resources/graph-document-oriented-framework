import { ColorWheel } from './ColorWheel.js';

export class CircleWheel extends ColorWheel {
  updateBackground(color, size) {
    if (!this.circleDiv) {
      this.containerEl.innerHTML = `
        <div class="cw-circle-layer" style="position:absolute; border-radius:50%;"></div>
        <div class="cw-thumb cw-picker-thumb"></div>
      `;
      this.circleDiv = this.querySelector('.cw-circle-layer');
      this.pickerThumb = this.querySelector('.cw-picker-thumb');
    }

    const center = this.getCenter(size);
    const outerRadius = this.getOuterRadius(size);
    const radius = outerRadius * 0.9;

    this.circleDiv.style.top = `${center - radius}px`;
    this.circleDiv.style.left = `${center - radius}px`;
    this.circleDiv.style.width = `${radius * 2}px`;
    this.circleDiv.style.height = `${radius * 2}px`;
    this.circleDiv.style.background = `radial-gradient(circle, white 0%, transparent 70%), conic-gradient(from 0deg, hsl(0,100%,50%), hsl(60,100%,50%), hsl(120,100%,50%), hsl(180,100%,50%), hsl(240,100%,50%), hsl(300,100%,50%), hsl(360,100%,50%))`;
  }

  updateThumbs(color, size) {
    const center = this.getCenter(size);
    const outerRadius = this.getOuterRadius(size);
    const radius = outerRadius * 0.9;
    const h = color[0], s = color[1];
    const angleRad = (h - 90.0) * Math.PI / 180.0;
    const dist = (s / 100) * radius;
    const px = center + dist * Math.cos(angleRad);
    const py = center + dist * Math.sin(angleRad);
    
    this.pickerThumb.style.left = `${px}px`;
    this.pickerThumb.style.top = `${py}px`;
    this.pickerThumb.style.backgroundColor = `hsla(${color[0]}, ${color[1]}%, ${color[2]}%, ${color[3]})`;
  }

  getColorFromCoords(x, y, size, mode) {
    const center = this.getCenter(size);
    const outerRadius = this.getOuterRadius(size);
    const radius = outerRadius * 0.9;
    const dx = x - center, dy = y - center;
    let angle = Math.atan2(dy, dx) * 180 / Math.PI + 90;
    if (angle < 0) angle += 360;
    const dist = Math.min(Math.sqrt(dx * dx + dy * dy), radius);
    const s = (dist / radius) * 100;
    const currentColor = this._color;
    return [angle, s, currentColor[2], currentColor[3]];
  }
}
customElements.define('circle-wheel', CircleWheel);