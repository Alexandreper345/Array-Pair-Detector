use std::collections::HashMap;

fn is_vetor_par(value: [i8; 5]) -> HashMap<i8,u8>
{
	let mut contador: HashMap<i8,u8> = HashMap::new();

	for num in value.iter()
	{
		*contador.entry(*num).or_insert(0)+=1;
	}
	for (_,repeticoes) in contador.iter_mut()
	{
		if *repeticoes > 0
		{
		   *repeticoes /= 2
		}
	}
	return contador
}

#[cfg(test)]
mod tests
{
	use crate::*;

	#[test]
	fn test_is_par()
	{
		let list:[i8;5] = [1,2,2,3,4];
		let repete = is_vetor_par(list);
		assert_eq!(repete[&2],1);
	}
	#[test]
	fn test_is_par2()
	{
		let list:[i8;5] = [1,2,3,4,5];
		let repete = is_vetor_par(list);
		assert_eq!(repete[&1],0)
	}
}
