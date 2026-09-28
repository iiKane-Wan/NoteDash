const pad = (n: number) => String(n).padStart(2, "0");

/** 列表时间展示：今天只显示时分，其余显示月-日 时分 */
export function formatTime(iso: string): string {
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return "";
  const now = new Date();
  const time = `${pad(d.getHours())}:${pad(d.getMinutes())}`;
  const sameDay = d.toDateString() === now.toDateString();
  return sameDay ? time : `${d.getMonth() + 1}-${pad(d.getDate())} ${time}`;
}