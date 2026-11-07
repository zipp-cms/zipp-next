// the live preview on or off, in this browser for every entry
const KEY = 'zipp-live-preview';
export const liveSetting = $state({
	on: localStorage.getItem(KEY) === '1',
});

export function toggleLive() {
	liveSetting.on = !liveSetting.on;
	localStorage.setItem(KEY, liveSetting.on ? '1' : '');
}
