
class Actions {
    constructor() {
        this.actions = []; // [[id, func]]    
    }

    execute() {
        if(this.actions.length <= 0) return;

        var act = this.actions[0];
        this.actions.shift();

        act[1]();
    }

    clear() {
        this.actions = [];
    }

    clear_key(ac) {
        this.actions = this.actions.filter(function (arr) {
            return arr[0] != ac
        });
    }

    has_key(ac) {
        var out = this.actions.find(function (arr) {
            return arr[0] == ac
        });
        return out != undefined;
    }

    add(id, func) {
        this.actions.push([id, func]);
    }
}