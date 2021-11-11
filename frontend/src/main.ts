import App from './App.svelte';

import dayjs from 'dayjs';
import localizedFormat from 'dayjs/plugin/localizedFormat';
dayjs.extend(localizedFormat)

const app = new App({
	target: document.body,
});

export default app;