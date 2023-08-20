
export class MyObject {
    constructor (value) {
        this.value = value;
    }
    get() {
        return this.value;
    }
    set(value) {
        this.value = value;
        console.log('value changed to', this.value);
    }
}
