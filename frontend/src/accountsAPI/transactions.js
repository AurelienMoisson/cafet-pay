export function newTransaction(student, selectedProducts, selectedReductions, total) {
  // should only accept transactions when the server calculates the same total
  // a transaction with positive total can be accepted even without being linked to a student.
  if (student===undefined && total!==0) {
    return false
  }
  return true
}
