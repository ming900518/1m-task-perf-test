const numTasks = 1_000_000;
const target = new Map();
const tasks = new Array();

for (let i = 0; i < numTasks; i++) {
    tasks.push(asyncTask(i));
}

Promise.all(tasks).then(() => {
    console.log(`${target.size} items pushed into target.`);
});

async function asyncTask(i) {
    target.set(i, i);
}
