import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { ActionBase } from "../action-base";
import { $ActionType, ActionType } from "../action-type";

export interface Greater extends ActionBase {
  action: ActionType.Greater;
}

export const $Greater: DocumentIoType<Greater> = new DocumentType<Greater>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.Greater as ActionType.Greater})},
  },
  changeCase: CaseStyle.SnakeCase,
});
