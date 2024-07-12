import com.sajjon.one.*
import com.sajjon.two.*

fun test() {
    assert(betaRecordToObject(record = newBetaRecordDefault()) == BetaObject.newDefault())
}

test()