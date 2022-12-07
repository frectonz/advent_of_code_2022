const content = await Deno.readFile("../data");
const decoder = new TextDecoder("utf-8");
const data = decoder.decode(content);

const elfCalories = data.split("\n\n");

const totalCalories = elfCalories.map((elf) => {
  const calories = elf.split("\n").map((calorie) => parseInt(calorie));
  const totalCalories = calories.reduce((a, b) => a + b, 0);
  return totalCalories;
});

const totalCaloriesSorted = totalCalories.sort((a, b) => b - a);

console.log(totalCaloriesSorted[0] + totalCaloriesSorted[1] + totalCaloriesSorted[2]);
