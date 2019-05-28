
class Node {
  constructor(prev, next, value) {
    this.prev = prev;
    this.next = next;
    this.value = value;
  }
}

export class LinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
  }

  push(value) {
    this.tail = new Node(this.tail, null, value);
    if (this.head === null) {
      this.head = this.tail;
    } else {
      this.tail.prev.next = this.tail;
    }
  }

  pop() {
    const node = this.tail;
    if (node === null) {
      throw new Error("Cannot pop from empty list");
    }
    this.tail = node.prev;
    if (this.tail !== null) {
      this.tail.next = null;
    } else {
      this.head = null;
    }
    return node.value;
  }

  shift() {
    const node = this.head;
    if (node === null) {
      throw new Error("Cannot shift from empty list");
    }
    this.head = node.next;
    if (this.head === null) {
      this.tail = null;
    }
    return node.value;
  }

  unshift(value) {
    this.head = new Node(null, this.head, value);
    if (this.tail === null) {
      this.tail = this.head;
    } else {
      this.head.next.prev = this.head;
    }
  }

  count() {
    let count = 0, node = this.head;
    while (node !== null) {
      count++;
      node = node.next;
    }
    return count;
  }

  delete(value) {
    let node = this.head;
    while (node !== null) {
      if (node.value === value) {
        if (node.prev !== null) {
          node.prev.next = node.next;
        }
        if (node.next !== null) {
          node.next.prev = node.prev;
        }
        if (this.head === node) {
          this.head = node.next;
        }
        if (this.tail === node) {
          this.tail = node.prev;
        }
        return;
      }
      node = node.next;
    }
  }
}
