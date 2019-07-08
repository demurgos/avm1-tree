import { CaseStyle } from "kryo/case-style";
import { ArrayType } from "kryo/types/array";
import { DocumentIoType, DocumentType } from "kryo/types/document";
import { $CfgAction, CfgAction } from "../cfg-action";
import { $CfgLabel, CfgLabel } from "../cfg-label";
import { $CfgBlockType, CfgBlockType } from "../cfg-block-type";
import { LiteralType } from "kryo/types/literal";

export interface CfgReturnBlock {
  readonly type: CfgBlockType.Return;
  readonly label: CfgLabel;
  readonly actions: ReadonlyArray<CfgAction>;
}

export const $CfgReturnBlock: DocumentIoType<CfgReturnBlock> = new DocumentType<CfgReturnBlock>(() => ({
  properties: {
    type: {type: new LiteralType({type: $CfgBlockType, value: CfgBlockType.Return as CfgBlockType.Return})},
    label: {type: $CfgLabel},
    actions: {type: new ArrayType({itemType: $CfgAction, maxLength: Infinity})},
  },
  changeCase: CaseStyle.SnakeCase,
}));