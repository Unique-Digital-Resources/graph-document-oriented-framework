export class PaletteGrid extends HTMLElement {
  constructor() {
    super();
    this.plates = [];
    this.activePlateId = 'plate-1';
    this.selectedSwatchIndex = -1;
    this.selectedSwatchType = null;
    this.dragSrcType = null;
    this.dragSrcIndex = null;
    this.tooltipEl = null;
    this.tooltipTimer = null;
    this.tooltipVisible = false;
    this.activeTooltipSwatch = null;
    this.TOOLTIP_DELAY = 700;
    this.dom = {};
  }

  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || nodeData.data || {};
    
    try {
      this.plates = JSON.parse(props.plates_json || "[]");
    } catch (e) {
      this.plates = [];
    }

    if (this.plates.length > 0 && !this.plates.find(p => p.id === this.activePlateId)) {
      this.activePlateId = this.plates[0].id;
    }

    this._currentDocumentColor = props.document_color || [0, 100, 50, 1];

    if (!this.dom.contentBox) {
      this.init();
    } else {
      this.renderTabs();
      this.renderContent();
      this.updateActionsState();
    }
  }

  getTargetNode() {
    return this._data?.props?.target_node || this._data?.data?.target_node || this._data?.target_node;
  }

  init() {
    this.renderShell();
    this.renderActions();
    this.renderTabs();
    this.renderContent();
    this.createTooltip();
    this.updateActionsState();
  }

  // --- Tooltip ---
  createTooltip() {
    this.tooltipEl = document.createElement('div');
    this.tooltipEl.className = 'swatch-tooltip';
    this.tooltipEl.style.display = 'none';
    document.body.appendChild(this.tooltipEl);
  }

  showTooltip(swatch, content) {
    this.tooltipEl.innerHTML = content;
    this.tooltipEl.classList.remove('tt-visible');
    this.tooltipEl.style.display = 'flex';

    const rect = swatch.getBoundingClientRect();
    const ttRect = this.tooltipEl.getBoundingClientRect();

    let left = rect.left + rect.width / 2 - ttRect.width / 2;
    let top = rect.top - ttRect.height - 6;

    if (left < 4) left = 4;
    if (left + ttRect.width > window.innerWidth - 4) left = window.innerWidth - ttRect.width - 4;
    if (top < 4) top = rect.bottom + 6;

    this.tooltipEl.style.left = `${left}px`;
    this.tooltipEl.style.top = `${top}px`;

    requestAnimationFrame(() => this.tooltipEl.classList.add('tt-visible'));
    this.tooltipVisible = true;
  }

  hideTooltip() {
    if (this.tooltipEl) {
      this.tooltipEl.classList.remove('tt-visible');
      setTimeout(() => {
        if (!this.tooltipVisible) this.tooltipEl.style.display = 'none';
      }, 160);
    }
    this.tooltipVisible = false;
    clearTimeout(this.tooltipTimer);
    this.tooltipTimer = null;
    this.activeTooltipSwatch = null;
  }

  scheduleTooltip(swatch, contentFn) {
    this.activeTooltipSwatch = swatch;
    clearTimeout(this.tooltipTimer);
    this.tooltipTimer = setTimeout(() => {
      if (this.activeTooltipSwatch === swatch) this.showTooltip(swatch, contentFn());
    }, this.TOOLTIP_DELAY);
  }

  getSwatchTooltipContent(type, data) {
    if (type === 'color') {
      const c = data;
      const hex = `#${Math.round(c[0]).toString(16).padStart(2,'0')}${Math.round(c[1]).toString(16).padStart(2,'0')}${Math.round(c[2]).toString(16).padStart(2,'0')}`;
      return `<span class="tt-hex">${hex}</span>`;
    } else if (type === 'harmony') {
      const dots = data.slice(-4).map(t => {
        const c = t;
        const hex = `#${Math.round(c[0]).toString(16).padStart(2,'0')}${Math.round(c[1]).toString(16).padStart(2,'0')}${Math.round(c[2]).toString(16).padStart(2,'0')}`;
        return `<span class="tt-dot" style="background:hsl(${c[0]},${c[1]}%,${c[2]}%)"></span><span class="tt-hex">${hex}</span>`;
      }).join('<div class="tt-sep"></div>');
      return dots;
    }
    return '';
  }

  bindSwatchTooltip(swatch, type, data) {
    const contentFn = () => this.getSwatchTooltipContent(type, data);
    swatch.addEventListener('mouseenter', () => this.scheduleTooltip(swatch, contentFn));
    swatch.addEventListener('mousemove', () => {
      if (this.activeTooltipSwatch === swatch && !this.tooltipVisible) this.scheduleTooltip(swatch, contentFn);
    });
    swatch.addEventListener('mouseleave', () => this.hideTooltip());
  }

  // --- Core ---
  getActivePlate() {
    return this.plates.find(p => p.id === this.activePlateId);
  }

  getSwatchBorderColors(h, s, l) {
    const isLight = l > 55;
    if (isLight) return { hover: '#4338ca', focus: '#b45309', selected: '#047857' };
    return { hover: '#a5b4fc', focus: '#fcd34d', selected: '#6ee7b7' };
  }

  renderShell() {
    // We no longer add an inner 'plate-system' div. 
    // The <palette-grid> element itself acts as the container.
    this.innerHTML = `
      <div class="plate-content" id="plate-content"></div>
      <div class="plate-left-section" style="width: 110px;">
        <div class="quick-actions-box" id="plate-quick-actions" style="flex-direction: row; gap: 4px; width: 100%;"></div>
        <div class="tabs-box" id="plate-tabs"></div>
      </div>
    `;
    this.dom.contentBox = this.querySelector('#plate-content');
    this.dom.tabsBox = this.querySelector('#plate-tabs');
    this.dom.quickActions = this.querySelector('#plate-quick-actions');
  }

  renderActions() {
    if (this.dom.quickActions.children.length > 0) return;

    const newTabBtn = document.createElement('button');
    newTabBtn.className = 'plate-action-btn';
    newTabBtn.innerHTML = '<span class="mdi mdi-plus"></span>';
    newTabBtn.title = "New Palette";
    newTabBtn.addEventListener('click', () => this.addPlate());
    this.dom.quickActions.appendChild(newTabBtn);

    const removeTabBtn = document.createElement('button');
    removeTabBtn.className = 'plate-action-btn danger';
    removeTabBtn.innerHTML = '<span class="mdi mdi-tab-remove"></span>';
    removeTabBtn.title = "Remove Palette";
    removeTabBtn.id = `remove-tab-btn`;
    removeTabBtn.addEventListener('click', () => this.removeActivePlate());
    this.dom.quickActions.appendChild(removeTabBtn);

    const removeSwatchBtn = document.createElement('button');
    removeSwatchBtn.className = 'plate-action-btn danger';
    removeSwatchBtn.innerHTML = '<span class="mdi mdi-delete"></span>';
    removeSwatchBtn.title = "Remove Selected Color";
    removeSwatchBtn.id = `remove-swatch-btn`;
    removeSwatchBtn.addEventListener('click', () => this.removeSelectedSwatch());
    this.dom.quickActions.appendChild(removeSwatchBtn);
  }

  updateActionsState() {
    const removeTabBtn = this.querySelector('#remove-tab-btn');
    const removeSwatchBtn = this.querySelector('#remove-swatch-btn');
    if (removeTabBtn) removeTabBtn.disabled = this.plates.length <= 1;
    if (removeSwatchBtn) removeSwatchBtn.disabled = this.selectedSwatchIndex === -1;
  }

  renderTabs() {
    this.dom.tabsBox.innerHTML = '';
    this.plates.forEach((plate, index) => {
      const tab = document.createElement('div');
      tab.className = `plate-tab ${plate.id === this.activePlateId ? 'active' : ''}`;
      tab.draggable = true;
      tab.dataset.index = index;

      const nameSpan = document.createElement('span');
      nameSpan.className = 'plate-tab-name';
      nameSpan.textContent = plate.name;
      tab.appendChild(nameSpan);

      tab.addEventListener('dragstart', () => {
        this.dragSrcType = 'tab';
        this.dragSrcIndex = index;
        tab.classList.add('dragging');
      });
      tab.addEventListener('dragend', () => {
        tab.classList.remove('dragging');
        this.dragSrcType = null;
        this.dragSrcIndex = null;
      });
      tab.addEventListener('dragover', (e) => e.preventDefault());
      tab.addEventListener('drop', (e) => {
        e.preventDefault();
        if (this.dragSrcType === 'tab') this.reorderTabs(this.dragSrcIndex, index);
      });

      tab.addEventListener('click', (e) => {
        if (e.target === nameSpan) {
          this.editTabName(plate, nameSpan);
          return;
        }
        this.activePlateId = plate.id;
        this.selectedSwatchIndex = -1;
        this.selectedSwatchType = null;
        this.hideTooltip();
        this.renderTabs();
        this.renderContent();
        this.updateActionsState();
      });

      this.dom.tabsBox.appendChild(tab);
    });
  }

  renderContent() {
    const plate = this.getActivePlate();
    this.dom.contentBox.innerHTML = '';
    const grid = document.createElement('div');
    grid.className = 'plate-colors-grid';

    const addBtn = document.createElement('div');
    addBtn.className = 'plate-swatch plate-add-btn';
    addBtn.innerHTML = '<span class="mdi mdi-plus"></span>';
    addBtn.addEventListener('click', () => this.addColor(this._currentDocumentColor));
    grid.appendChild(addBtn);

    if (plate) {
      plate.colors.forEach((color, index) => {
        const swatch = document.createElement('div');
        const isSelected = this.selectedSwatchIndex === index && this.selectedSwatchType === 'color';
        swatch.className = `plate-swatch ${isSelected ? 'selected' : ''}`;
        swatch.style.backgroundColor = `hsla(${color[0]}, ${color[1]}%, ${color[2]}%, ${color[3]})`;
        swatch.draggable = true;
        swatch.dataset.index = index;
        swatch.dataset.type = 'color';

        const bColors = this.getSwatchBorderColors(color[0], color[1], color[2]);
        swatch.style.setProperty('--swatch-hover', bColors.hover);
        swatch.style.setProperty('--swatch-focus', bColors.focus);
        swatch.style.setProperty('--swatch-selected', bColors.selected);

        this.bindSwatchTooltip(swatch, 'color', color);

        swatch.addEventListener('click', () => {
          this.hideTooltip();
          this.selectedSwatchIndex = index;
          this.selectedSwatchType = 'color';
          this.renderContent();
          this.updateActionsState();
          this.loadSwatch(color);
        });

        swatch.addEventListener('dragstart', () => {
          this.dragSrcType = 'color';
          this.dragSrcIndex = index;
          swatch.style.opacity = '0.5';
        });
        swatch.addEventListener('dragend', () => {
          swatch.style.opacity = '1';
          this.dragSrcType = null;
          this.dragSrcIndex = null;
        });
        swatch.addEventListener('dragover', (e) => e.preventDefault());
        swatch.addEventListener('drop', (e) => {
          e.preventDefault();
          if (this.dragSrcType === 'color') this.reorderSwatches(this.dragSrcIndex, index, 'color');
        });

        grid.appendChild(swatch);
      });

      plate.harmonies.forEach((harmony, index) => {
        const swatch = document.createElement('div');
        const isSelected = this.selectedSwatchIndex === index && this.selectedSwatchType === 'harmony';
        swatch.className = `plate-swatch harmony-swatch ${isSelected ? 'selected' : ''}`;
        swatch.draggable = true;
        swatch.dataset.index = index;
        swatch.dataset.type = 'harmony';

        const baseThumb = harmony.find(t => t.isBase) || harmony[0];
        if (baseThumb) {
          swatch.style.backgroundColor = `hsl(${baseThumb[0]}, ${baseThumb[1]}%, ${baseThumb[2]}%)`;
          const hbColors = this.getSwatchBorderColors(baseThumb[0], baseThumb[1], baseThumb[2]);
          swatch.style.setProperty('--swatch-hover', hbColors.hover);
          swatch.style.setProperty('--swatch-focus', hbColors.focus);
          swatch.style.setProperty('--swatch-selected', hbColors.selected);
        }

        const dotsGrid = document.createElement('div');
        dotsGrid.className = 'harmony-dots-grid';
        const lastThumbs = harmony.slice(-4);
        for (let i = 0; i < 4; i++) {
          const dot = document.createElement('div');
          dot.className = 'harmony-color-dot';
          if (i < lastThumbs.length) {
            const t = lastThumbs[i];
            dot.style.backgroundColor = `hsl(${t[0]}, ${t[1]}%, ${t[2]}%)`;
          }
          dotsGrid.appendChild(dot);
        }
        swatch.appendChild(dotsGrid);

        this.bindSwatchTooltip(swatch, 'harmony', harmony);

        swatch.addEventListener('click', () => {
          this.hideTooltip();
          this.selectedSwatchIndex = index;
          this.selectedSwatchType = 'harmony';
          this.renderContent();
          this.updateActionsState();
          this.loadSwatch(harmony);
        });

        swatch.addEventListener('dragstart', () => {
          this.dragSrcType = 'harmony';
          this.dragSrcIndex = index;
          swatch.style.opacity = '0.5';
        });
        swatch.addEventListener('dragend', () => {
          swatch.style.opacity = '1';
          this.dragSrcType = null;
          this.dragSrcIndex = null;
        });
        swatch.addEventListener('dragover', (e) => e.preventDefault());
        swatch.addEventListener('drop', (e) => {
          e.preventDefault();
          if (this.dragSrcType === 'harmony') this.reorderSwatches(this.dragSrcIndex, index, 'harmony');
        });

        grid.appendChild(swatch);
      });
    }
    this.dom.contentBox.appendChild(grid);
  }

  // --- Dispatch Commands to Rust ---
  addPlate() {
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'AddPalettePlate',
      params: { target_node: this.getTargetNode() }
    }}));
  }

  removeActivePlate() {
    if (this.plates.length <= 1) return;
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'RemovePalettePlate',
      params: { target_node: this.getTargetNode(), plate_id: this.activePlateId }
    }}));
    this.activePlateId = this.plates[0].id;
  }

  removeSelectedSwatch() {
    if (this.selectedSwatchIndex === -1) return;
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'RemoveSwatch',
      params: { 
        target_node: this.getTargetNode(), 
        plate_id: this.activePlateId,
        type: this.selectedSwatchType,
        index: this.selectedSwatchIndex
      }
    }}));
    this.selectedSwatchIndex = -1;
  }

  addColor(color) {
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'AddColorToPlate',
      params: { target_node: this.getTargetNode(), plate_id: this.activePlateId, swatch: color }
    }}));
  }

  loadSwatch(swatch) {
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'LoadSwatchToDocument',
      params: { target_node: this.getTargetNode(), swatch: swatch }
    }}));
  }

  reorderTabs(from, to) {
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'ReorderPlates',
      params: { target_node: this.getTargetNode(), from: from, to: to }
    }}));
  }

  reorderSwatches(from, to, type) {
    window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
      type: 'custom', target: this.id, command_id: 'ReorderSwatches',
      params: { target_node: this.getTargetNode(), plate_id: this.activePlateId, type: type, from: from, to: to }
    }}));
  }

  editTabName(plate, nameSpan) {
    const input = document.createElement('input');
    input.type = 'text';
    input.className = 'editable-input';
    input.value = plate.name;
    
    const commit = () => {
      const newName = input.value.trim() || 'Untitled';
      window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
        type: 'custom', target: this.id, command_id: 'RenamePalettePlate',
        params: { target_node: this.getTargetNode(), plate_id: plate.id, name: newName }
      }}));
    };

    input.addEventListener('blur', commit);
    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') commit();
      if (e.key === 'Escape') this.renderTabs();
    });

    nameSpan.replaceWith(input);
    input.focus();
    input.select();
  }
}
customElements.define('palette-grid', PaletteGrid);