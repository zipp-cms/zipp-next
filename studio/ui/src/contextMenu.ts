/**
 * Takes a right click for a menu: false when another menu took it
 * already or a text field keeps the browser's menu (paste, spelling),
 * else the browser's menu and the menus of outer elements stay closed.
 */
export function takeContextMenu(e: MouseEvent): boolean {
	if (e.defaultPrevented) return false;
	if ((e.target as Element).closest('input, textarea, [contenteditable]'))
		return false;
	e.preventDefault();
	e.stopPropagation();
	return true;
}

/** The pointer of a right click, null for the menu key (no pointer). */
export function pointerOf(e: MouseEvent): { x: number; y: number } | null {
	return e.button === 2 || e.clientX ? { x: e.clientX, y: e.clientY } : null;
}
