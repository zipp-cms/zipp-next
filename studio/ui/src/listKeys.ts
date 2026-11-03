/**
 * The item the arrow keys, Home and End move the focus to in a list of
 * `n` from `i` (-1: none has it), undefined for other keys. A menu goes
 * round, a long list stops at its ends.
 */
export function listKey(
	key: string,
	i: number,
	n: number,
	round = false,
): number | undefined {
	if (!n) return;
	switch (key) {
		case 'ArrowDown':
			return round ? (i + 1) % n : Math.min(i + 1, n - 1);
		case 'ArrowUp':
			if (round) return i <= 0 ? n - 1 : i - 1;
			return Math.max(i - 1, 0);
		case 'Home':
			return 0;
		case 'End':
			return n - 1;
	}
}
