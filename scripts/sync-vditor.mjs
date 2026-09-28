// 将 Vditor 运行时所需资源从 node_modules 同步到 public/vditor，
// 使编辑器在完全离线的环境下也能正常渲染（图标、代码高亮、Lute 解析器等）。
import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { dirname } from "node:path";

const source = "node_modules/vditor/dist";
const target = "public/vditor/dist";
const marker = `${target}/js/lute/lute.min.js`;

if (existsSync(marker)) {
  console.log("[vditor] 本地资源已就绪，跳过同步");
  process.exit(0);
}

if (!existsSync(source)) {
  console.error("[vditor] 未找到 node_modules/vditor，请先执行 npm install");
  process.exit(1);
}

// 只同步实际用到的资源，剔除 mermaid/katex/echarts 等体积较大的可选依赖
const items = [
  "css",
  "images",
  "js/lute",
  "js/icons",
  "js/highlight.js",
  "js/i18n",
];

rmSync("public/vditor", { recursive: true, force: true });
for (const item of items) {
  const dest = `${target}/${item}`;
  mkdirSync(dirname(dest), { recursive: true });
  cpSync(`${source}/${item}`, dest, { recursive: true });
}

console.log(`[vditor] 已同步 ${items.length} 组资源到 ${target}`);
