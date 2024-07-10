import two

public func test() throws {
  assert(newRecordDefault() == newRecordDefault())
  assert(newRecordDefault() == newRecord(one: newOneDefault(), two: newTwoDefault()))
  assert(AlphaObject.newDefault() == AlphaObject.newDefault())
  assert(objectRecord(object: AlphaObject.newDefault()) == newRecordDefault())
  assert(recordObject(record: newRecordDefault()) == AlphaObject.newDefault())

  do {
    let r = newRecord(one: newOne(value: true), two: newTwo(value: true))
    assert(r == recordRecord(record: r))
    assert(r == recordRefRecord(record: r))
  }

  do {
    let o = AlphaObject(one: newOne(value: true), two: newTwo(value: true))
    assert(o == objectObject(object: o))
    assert(o == objectRefObject(object: o))
  }

}

try! test()
