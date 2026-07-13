import { ColorWheel } from './ColorWheel.js';

export class TriangleWheel extends ColorWheel {
  updateBackground(color, size) {
    if (!this.hueRing) {
      this.containerEl.innerHTML = `
        <div class="cw-hue-ring"></div>
        <div class="cw-hue-mask"></div>
        <svg class="cw-triangle-layer" viewBox="0 0 ${size} ${size}">
          <defs>
            <linearGradient id="tri-sat" gradientUnits="userSpaceOnUse"><stop offset="0%" stop-color="red"/><stop offset="100%" stop-color="white"/></linearGradient>
            <linearGradient id="tri-light" gradientUnits="userSpaceOnUse"><stop offset="0%" stop-color="black"/><stop offset="100%" stop-color="transparent"/></linearGradient>
          </defs>
          <polygon fill="url(#tri-sat)"></polygon>
          <polygon fill="url(#tri-light)" opacity="1.0"></polygon>
        </svg>
        <div class="cw-thumb cw-hue-thumb"></div>
        <div class="cw-thumb cw-picker-thumb"></div>
      `;
      this.hueRing = this.querySelector('.cw-hue-ring');
      this.hueMask = this.querySelector('.cw-hue-mask');
      this.triSvg = this.querySelector('.cw-triangle-layer');
      this.triPoly1 = this.triSvg.querySelectorAll('polygon')[0];
      this.triPoly2 = this.triSvg.querySelectorAll('polygon')[1];
      this.satGrad = this.triSvg.querySelector('#tri-sat');
      this.lightGrad = this.triSvg.querySelector('#tri-light');
      this.satStop = this.satGrad.querySelectorAll('stop')[0];
      this.hueThumb = this.querySelector('.cw-hue-thumb');
      this.pickerThumb = this.querySelector('.cw-picker-thumb');
    }

    // FIX: Always update sizes and SVG coordinates
    const center = this.getCenter(size);
    const outerRadius = this.getOuterRadius(size);
    const innerRadius = this.getInnerRadius(size);
    const radius = innerRadius * 0.85;

    this.hueRing.style.width = `${outerRadius * 2}px`;
    this.hueRing.style.height = `${outerRadius * 2}px`;
    this.hueRing.style.left = `${center - outerRadius}px`;
    this.hueRing.style.top = `${center - outerRadius}px`;

    this.hueMask.style.width = `${innerRadius * 2}px`;
    this.hueMask.style.height = `${innerRadius * 2}px`;
    this.hueMask.style.left = `${center - innerRadius}px`;
    this.hueMask.style.top = `${center - innerRadius}px`;

    this.triSvg.setAttribute('viewBox', `0 0 ${size} ${size}`);

    const angle = (color[0] - 90.0) * Math.PI / 180.0;
    const x1 = center + radius * Math.cos(angle), y1 = center + radius * Math.sin(angle);
    const x2 = center + radius * Math.cos(angle + 2.0 * Math.PI / 3.0), y2 = center + radius * Math.sin(angle + 2.0 * Math.PI / 3.0);
    const x3 = center + radius * Math.cos(angle + 4.0 * Math.PI / 3.0), y3 = center + radius * Math.sin(angle + 4.0 * Math.PI / 3.0);
    const pointsStr = `${x1},${y1} ${x2},${y2} ${x3},${y3}`;

    this.triPoly1.setAttribute("points", pointsStr);
    this.triPoly2.setAttribute("points", pointsStr);
    this.satStop.setAttribute("stop-color", `hsl(${color[0]}, 100%, 50%)`);

    this.satGrad.setAttribute("x1", x1); this.satGrad.setAttribute("y1", y1);
    this.satGrad.setAttribute("x2", x2); this.satGrad.setAttribute("y2", y2);
    this.lightGrad.setAttribute("x1", x3); this.lightGrad.setAttribute("y1", y3);
    this.lightGrad.setAttribute("x2", (x1 + x2) / 2.0); this.lightGrad.setAttribute("y2", (y1 + y2) / 2.0);
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

    const radius = innerRadius * 0.85;
    const h = color[0], s = color[1] / 100, l = color[2] / 100;
    const a = s, b = l - 0.5 * s, c = 1 - a - b;
    let na = Math.max(0, a), nb = Math.max(0, b), nc = Math.max(0, c);
    let sum = na + nb + nc;
    if (sum > 0) { na /= sum; nb /= sum; nc /= sum; } else { na = 1; }
    
    const x1 = center + radius * Math.cos(angleRad), y1 = center + radius * Math.sin(angleRad);
    const x2 = center + radius * Math.cos(angleRad + 2.0 * Math.PI / 3.0), y2 = center + radius * Math.sin(angleRad + 2.0 * Math.PI / 3.0);
    const x3 = center + radius * Math.cos(angleRad + 4.0 * Math.PI / 3.0), y3 = center + radius * Math.sin(angleRad + 4.0 * Math.PI / 3.0);
    
    const px = na * x1 + nb * x2 + nc * x3;
    const py = na * y1 + nb * y2 + nc * y3;
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
      const radius = innerRadius * 0.85;
      const hue = currentColor[0];
      const angle = (hue - 90) * Math.PI / 180;
      const x1 = center + radius * Math.cos(angle), y1 = center + radius * Math.sin(angle);
      const x2 = center + radius * Math.cos(angle + 2 * Math.PI / 3), y2 = center + radius * Math.sin(angle + 2 * Math.PI / 3);
      const x3 = center + radius * Math.cos(angle + 4 * Math.PI / 3), y3 = center + radius * Math.sin(angle + 4 * Math.PI / 3);
      const denom = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
      if (Math.abs(denom) < 0.0001) return null;
      let a = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) / denom;
      let b = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) / denom;
      let c = 1 - a - b;
      a = Math.max(0, a); b = Math.max(0, b); c = Math.max(0, c);
      let sum = a + b + c;
      if (sum > 0) { a /= sum; b /= sum; c /= sum; } else { a = 1; }
      const s = 100 * a;
      const l = 50 * a + 100 * b;
      return [hue, s, l, currentColor[3]];
    }
  }
}
customElements.define('triangle-wheel', TriangleWheel);