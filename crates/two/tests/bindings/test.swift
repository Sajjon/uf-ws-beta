import two

public func test() throws {
  assert(betaRecordToObject(record: newBetaRecordDefault()) == BetaObject.newDefault())
}

try! test()
