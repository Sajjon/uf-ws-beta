import two

public func test() throws {
  assert(newRecordDefault() == newRecordDefault())
  assert(newRecordDefault() == newRecord(one: newOneDefault(), two: newTwoDefault()))
  assert(AlphaObject.newDefault() == AlphaObject.newDefault())
  assert(objectRecord(object: AlphaObject.newDefault()) == newRecordDefault())
  assert(recordObject(record: newRecordDefault()) == AlphaObject.newDefault())
}

try! test()
