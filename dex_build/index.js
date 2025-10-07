/*
 * This file is a simple script I used to print out all pokemon names, 
 * abilities and moves to build tables from txt files.
 */


import { strict as assert } from 'node:assert';
import {Dex} from '@pkmn/dex';

const allTypes = Dex.forGen(9).types.all();
//let asd = JSON.stringify(allTypes, null, 2)
//console.log(asd);

// so I take all the types, go through each one for attack
const n = allTypes.length;
for(let i = 0; i < n; i++) {
    let attackType = allTypes[i];
    //console.log(attackType.name);
    // then I take the same loop just for defense
    for(let j = 0; j < n; j++) {
        let defenseType = allTypes[j];
        let d = defenseType.name;
        if (defenseType.damageTaken.hasOwnProperty(attackType.name)) {
            console.log(`
                attack: ${attackType.name} 
                defense: ${defenseType.name}
                effect: ${defenseType.damageTaken[d]}
            `);
            console.log(defenseType.damageTaken);
        }
    }
}





function printType(type) {
    console.log(`
        ${type.name}
    `);
    console.log(type.damageTaken);
}

//const allPokemon = Dex.forGen(9).species.all();
//// c for loop cause its easier for me conceptually
//for(let i = 0; i < allPokemon.length; i++) {
//    let pokemon = allPokemon[i];
//    console.log(pokemon.name);
//}
//console.log(`Names: ${allPokemon.length}`);

//const allItems = Dex.forGen(9).items.all();
//allItems.sort((a, b) => a.num - b.num);
//for(let i = 0; i < allItems.length; i++) {
//    let item = allItems[i];
//    console.log(item.name);
//}
//console.log(`Items: ${allItems.length}`);

//const allAbilities = Dex.forGen(9).abilities.all();
//// sort by number for better gen compatability?
//allAbilities.sort((a, b) => a.num - b.num);
//for(let i = 0; i < allAbilities.length; i++) {
//    let ability = allAbilities[i];
//    console.log(ability.name);
//}
//console.log(`Abilities: ${allAbilities.length}`);

//const allMoves = Dex.forGen(9).moves.all();
//allMoves.sort((a, b) => a.num - b.num);
//for(let i = 0; i < allMoves.length; i++) {
//    let move = allMoves[i];
//    console.log(move.name);
//}
//console.log(`Moves: ${allMoves.length}`);

//console.log("test");
//assert(Dex.forGen(1).types.get('Psychic').damageTaken['Ghost'] === 3);
//assert(Dex.getEffectiveness('Dark', ['Ghost', 'Psychic']) === 2);
//assert(Dex.forGen(5).species.get('Dragapult').isNonstandard === 'Future');
//assert(Dex.forGen(3).species.get('Chansey').prevo === 'Happiny');
//assert(Dex.forGen(1).species.all().filter(s => !s.isNonstandard).length === 151);

