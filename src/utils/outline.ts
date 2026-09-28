/** 大纲项：直接持有标题元素引用，点击即可精确滚动定位 */
export interface OutlineItem {
  id: number;
  level: number;
  text: string;
  el: HTMLElement;
}

/** 仅保留可见标题，排除 Vditor 内部辅助节点 */
function isVisible(el: HTMLElement): boolean {
  return el.offsetParent !== null;
}

/** 提取标题纯文本，剔除 Vditor 的 Markdown 标记节点（如 "## "） */
function headingText(el: HTMLElement): string {
  const clone = el.cloneNode(true) as HTMLElement;
  clone.querySelectorAll(".vditor-ir__marker, .vditor-ir__preview").forEach((node) => {
    node.remove();
  });
  return (clone.textContent ?? "").replace(/\s+/g, " ").trim();
}

/** 按文档顺序收集编辑器内所有标题，生成大纲列表 */
export function collectOutline(root: HTMLElement): OutlineItem[] {
  const headings = Array.from(
    root.querySelectorAll<HTMLElement>("h1, h2, h3, h4, h5, h6"),
  ).filter(isVisible);

  const items: OutlineItem[] = [];
  for (const el of headings) {
    const text = headingText(el);
    if (!text) continue;
    items.push({
      id: items.length,
      level: Number(el.tagName.slice(1)),
      text,
      el,
    });
  }
  return items;
}