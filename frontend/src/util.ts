export function dateToYMD(date: Date) {
    if(!date) { return ""; }
    return date.toISOString().split("T")[0];
}

export function pickMap(obj, key) {
    return Object.keys(obj).reduce((a, b) => {
        a[b] = obj[b][key];
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

export function addDays(days: number): Date {
    var date = new Date(); // TODO: get start of current week
    date.setDate(date.getDate() + days);
    return date;
};