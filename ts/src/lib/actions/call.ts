import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { $ActionType, ActionType } from "../action-type";
import { ActionBase } from "./_base";

export interface Call extends ActionBase {
  action: ActionType.Call;
}

export const $Call: DocumentIoType<Call> = new DocumentType<Call>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.Call as ActionType.Call})},
  },
  changeCase: CaseStyle.SnakeCase,
});
