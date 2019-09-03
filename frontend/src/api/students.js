function getStudents() {
  return []
}

function getStudentById(studentId) {
  return {
    name:"Alexandre Dupont",
    studentId:"studentId",
    // ...
    // other info ?
    // (like email, year...)
    // (But only if accessible
  }
}

function findStudentsByName(nameStart) {
  // Not case sensitive please
  return [{
    name:"${nameStart}exandre Dupont",
    studentId:"studentId",
    // ...
    // other info ?
    // (like email, year...)
    // (But only if accessible
  }]
}
