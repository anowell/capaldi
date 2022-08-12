import dayjs from 'dayjs';
import localizedFormat from 'dayjs/plugin/localizedFormat';
import isoWeek from 'dayjs/plugin/isoWeek';

// dayjs.extend(isoWeek);
dayjs.extend(localizedFormat);

declare global {
	interface Window {
        dayjs:any;
    }
}


import App from './App.svelte';
window.dayjs = dayjs;

const app = new App({
	target: document.body,
});

export default app;