export function toRna(dna: string): string {
  const nucleotides = [...dna];
  return nucleotides.map((n): string => {
    switch (n) {
      case 'A':
        return 'U';
      case 'T':
        return 'A';
      case 'G':
        return 'C';
      case 'C':
        return 'G';
      default:
        throw new Error('Invalid input DNA.')
    }
  }).join('')
}
