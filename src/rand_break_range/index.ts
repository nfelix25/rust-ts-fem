function main(): void {
  let color = "#555555";
  console.log("%cHello, World!", `color: ${color}; font-size: 40px;`);

  let i = 0;
  for (;;) {
    const rand = Math.floor(10 * Math.random());
    console.log(`This loop will run forever! #${i}`);
    if (i % 10 === rand) {
      console.log(`Breaking at i=${i} with rand=${rand}`);
      break;
    }
    i++;
  }

  console.log(`Exited loop at i=${i}`);
}

main();
