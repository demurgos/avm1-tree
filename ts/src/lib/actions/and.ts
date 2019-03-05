import { CaseStyle } from "kryo/case-style";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { LiteralType } from "kryo/types/literal";
import { ActionBase } from "../action-base";
import { $ActionType, ActionType } from "../action-type";

export interface And extends ActionBase {
  action: ActionType.And;
}

export const $And: DocumentIoType<And> = new DocumentType<And>({
  properties: {
    action: {type: new LiteralType({type: $ActionType, value: ActionType.And as ActionType.And})},
  },
  changeCase: CaseStyle.SnakeCase,
});
