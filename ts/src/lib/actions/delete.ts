import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { ActionBase } from "../action-base";
import { $ActionType, ActionType } from "../action-type";

export interface Delete extends ActionBase {
  action: ActionType.Delete;
}

export const $Delete: DocumentIoType<Delete> = new DocumentType<Delete>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.Delete as ActionType.Delete})},
  },
  changeCase: CaseStyle.SnakeCase,
});
