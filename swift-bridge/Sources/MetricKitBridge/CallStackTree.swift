import Foundation
import MetricKit

func mxCallStackTree(_ tree: MXCallStackTree) -> Any {
    mxJSONObject(from: tree.jsonRepresentation())
}
