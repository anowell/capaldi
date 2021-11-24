import dayjs, {Dayjs} from 'dayjs';

export function fmtDate(date: Date | Dayjs) {
    return dayjs(date).format("YYYY-MM-DD");
}

export function idMap(arr: {id:number|string}[]) {
    return arr.reduce((a, b) => {
        a[b.id] = b;
        return a;
    }, {});
};

export function groupB<T, K extends keyof any>(list: T[], getKey: (item: T) => K): Record<K, T[]> {
    return list.reduce((previous, currentItem) => {
        const group = getKey(currentItem);
        if (!previous[group]) previous[group] = [];
        previous[group].push(currentItem);
        return previous;
    }, {} as Record<K, T[]>);
}


export const currentWeek = dayjs().startOf("week").add(1, "day");
export function isCurrentWeek(date: Date): boolean {
    return currentWeek.isSame(date, "day");
}

export const waitForElement = async (selector: string, rootElement = document.documentElement) => {
    return new Promise((resolve) => {
        const observer = new MutationObserver(() => {
            const element = document.querySelector(selector);
            if (element) {
                observer.disconnect();
                resolve(element);
            }
        });

        observer.observe(rootElement, {
            childList: true,
            subtree: true,
        });
    });
};