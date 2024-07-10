import com.sajjon.one.*
import com.sajjon.two.*

fun test() {
    assert(newRecordDefault() == newRecordDefault())
    assert(newRecordDefault() == newRecord(one = newOneDefault(), two = newTwoDefault()))
    assert(BetaObject.newDefault() == BetaObject.newDefault())
    assert(objectRecord(value = BetaObject.newDefault()) == newRecordDefault())
    assert(recordObject(value = newRecordDefault()) == BetaObject.newDefault())
  
    val r = newRecord(one = newOne(value = true), two = newTwo(value = true))
    assert(r == recordRecord(value = r))
    assert(r == recordRefRecord(value = r))
  
    val o = BetaObject(one = newOne(value = true), two = newTwo(value = true))
    assert(o == objectObject(value = o))
    assert(o == objectRefObject(value = o))
}

test()