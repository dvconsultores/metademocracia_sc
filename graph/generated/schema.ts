// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  TypedMap,
  Entity,
  Value,
  ValueKind,
  store,
  Bytes,
  BigInt,
  BigDecimal
} from "@graphprotocol/graph-ts";

export class Proposaldata extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Proposaldata entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Proposaldata must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Proposaldata", id.toString(), this);
    }
  }

  static load(id: string): Proposaldata | null {
    return changetype<Proposaldata | null>(store.get("Proposaldata", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get proposal_actives(): BigInt {
    let value = this.get("proposal_actives");
    return value!.toBigInt();
  }

  set proposal_actives(value: BigInt) {
    this.set("proposal_actives", Value.fromBigInt(value));
  }

  get proposal_total(): BigInt {
    let value = this.get("proposal_total");
    return value!.toBigInt();
  }

  set proposal_total(value: BigInt) {
    this.set("proposal_total", Value.fromBigInt(value));
  }

  get roles(): string | null {
    let value = this.get("roles");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set roles(value: string | null) {
    if (!value) {
      this.unset("roles");
    } else {
      this.set("roles", Value.fromString(<string>value));
    }
  }

  get vote_policy(): string | null {
    let value = this.get("vote_policy");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set vote_policy(value: string | null) {
    if (!value) {
      this.unset("vote_policy");
    } else {
      this.set("vote_policy", Value.fromString(<string>value));
    }
  }

  get proposal_bond(): string | null {
    let value = this.get("proposal_bond");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set proposal_bond(value: string | null) {
    if (!value) {
      this.unset("proposal_bond");
    } else {
      this.set("proposal_bond", Value.fromString(<string>value));
    }
  }

  get proposal_period(): string | null {
    let value = this.get("proposal_period");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set proposal_period(value: string | null) {
    if (!value) {
      this.unset("proposal_period");
    } else {
      this.set("proposal_period", Value.fromString(<string>value));
    }
  }
}

export class Proposal extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Proposal entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Proposal must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Proposal", id.toString(), this);
    }
  }

  static load(id: string): Proposal | null {
    return changetype<Proposal | null>(store.get("Proposal", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get title(): string {
    let value = this.get("title");
    return value!.toString();
  }

  set title(value: string) {
    this.set("title", Value.fromString(value));
  }

  get description(): string {
    let value = this.get("description");
    return value!.toString();
  }

  set description(value: string) {
    this.set("description", Value.fromString(value));
  }

  get proposal_type(): string {
    let value = this.get("proposal_type");
    return value!.toString();
  }

  set proposal_type(value: string) {
    this.set("proposal_type", Value.fromString(value));
  }

  get kind(): string {
    let value = this.get("kind");
    return value!.toString();
  }

  set kind(value: string) {
    this.set("kind", Value.fromString(value));
  }

  get proposer(): string {
    let value = this.get("proposer");
    return value!.toString();
  }

  set proposer(value: string) {
    this.set("proposer", Value.fromString(value));
  }

  get submission_time(): BigInt {
    let value = this.get("submission_time");
    return value!.toBigInt();
  }

  set submission_time(value: BigInt) {
    this.set("submission_time", Value.fromBigInt(value));
  }

  get upvote(): BigInt {
    let value = this.get("upvote");
    return value!.toBigInt();
  }

  set upvote(value: BigInt) {
    this.set("upvote", Value.fromBigInt(value));
  }

  get downvote(): BigInt {
    let value = this.get("downvote");
    return value!.toBigInt();
  }

  set downvote(value: BigInt) {
    this.set("downvote", Value.fromBigInt(value));
  }

  get vote(): Array<string> {
    let value = this.get("vote");
    return value!.toStringArray();
  }

  set vote(value: Array<string>) {
    this.set("vote", Value.fromStringArray(value));
  }

  get status(): string {
    let value = this.get("status");
    return value!.toString();
  }

  set status(value: string) {
    this.set("status", Value.fromString(value));
  }

  get approval_date(): BigInt | null {
    let value = this.get("approval_date");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set approval_date(value: BigInt | null) {
    if (!value) {
      this.unset("approval_date");
    } else {
      this.set("approval_date", Value.fromBigInt(<BigInt>value));
    }
  }

  get creation_date(): BigInt {
    let value = this.get("creation_date");
    return value!.toBigInt();
  }

  set creation_date(value: BigInt) {
    this.set("creation_date", Value.fromBigInt(value));
  }

  get user_creation(): string {
    let value = this.get("user_creation");
    return value!.toString();
  }

  set user_creation(value: string) {
    this.set("user_creation", Value.fromString(value));
  }

  get link(): string {
    let value = this.get("link");
    return value!.toString();
  }

  set link(value: string) {
    this.set("link", Value.fromString(value));
  }

  get admin_appoved(): boolean {
    let value = this.get("admin_appoved");
    return value!.toBoolean();
  }

  set admin_appoved(value: boolean) {
    this.set("admin_appoved", Value.fromBoolean(value));
  }
}

export class Vote extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Vote entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Vote must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Vote", id.toString(), this);
    }
  }

  static load(id: string): Vote | null {
    return changetype<Vote | null>(store.get("Vote", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get proposal(): string {
    let value = this.get("proposal");
    return value!.toString();
  }

  set proposal(value: string) {
    this.set("proposal", Value.fromString(value));
  }

  get user_id(): string {
    let value = this.get("user_id");
    return value!.toString();
  }

  set user_id(value: string) {
    this.set("user_id", Value.fromString(value));
  }

  get vote(): string {
    let value = this.get("vote");
    return value!.toString();
  }

  set vote(value: string) {
    this.set("vote", Value.fromString(value));
  }

  get date_time(): BigInt {
    let value = this.get("date_time");
    return value!.toBigInt();
  }

  set date_time(value: BigInt) {
    this.set("date_time", Value.fromBigInt(value));
  }
}

export class Delegation extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Delegation entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Delegation must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Delegation", id.toString(), this);
    }
  }

  static load(id: string): Delegation | null {
    return changetype<Delegation | null>(store.get("Delegation", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get delegators(): Array<string> {
    let value = this.get("delegators");
    return value!.toStringArray();
  }

  set delegators(value: Array<string>) {
    this.set("delegators", Value.fromStringArray(value));
  }

  get history(): Array<string> {
    let value = this.get("history");
    return value!.toStringArray();
  }

  set history(value: Array<string>) {
    this.set("history", Value.fromStringArray(value));
  }

  get total_amount(): BigInt {
    let value = this.get("total_amount");
    return value!.toBigInt();
  }

  set total_amount(value: BigInt) {
    this.set("total_amount", Value.fromBigInt(value));
  }
}

export class Delegationhist extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Delegationhist entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Delegationhist must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Delegationhist", id.toString(), this);
    }
  }

  static load(id: string): Delegationhist | null {
    return changetype<Delegationhist | null>(store.get("Delegationhist", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get delegation(): string {
    let value = this.get("delegation");
    return value!.toString();
  }

  set delegation(value: string) {
    this.set("delegation", Value.fromString(value));
  }

  get date_time(): BigInt {
    let value = this.get("date_time");
    return value!.toBigInt();
  }

  set date_time(value: BigInt) {
    this.set("date_time", Value.fromBigInt(value));
  }

  get delegator(): string {
    let value = this.get("delegator");
    return value!.toString();
  }

  set delegator(value: string) {
    this.set("delegator", Value.fromString(value));
  }

  get amount(): BigInt {
    let value = this.get("amount");
    return value!.toBigInt();
  }

  set amount(value: BigInt) {
    this.set("amount", Value.fromBigInt(value));
  }
}

export class Delegator extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Delegator entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Delegator must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Delegator", id.toString(), this);
    }
  }

  static load(id: string): Delegator | null {
    return changetype<Delegator | null>(store.get("Delegator", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get delegation(): string {
    let value = this.get("delegation");
    return value!.toString();
  }

  set delegation(value: string) {
    this.set("delegation", Value.fromString(value));
  }

  get delegator(): string {
    let value = this.get("delegator");
    return value!.toString();
  }

  set delegator(value: string) {
    this.set("delegator", Value.fromString(value));
  }

  get amount(): BigInt {
    let value = this.get("amount");
    return value!.toBigInt();
  }

  set amount(value: BigInt) {
    this.set("amount", Value.fromBigInt(value));
  }
}

export class Fundshist extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Fundshist entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Fundshist must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Fundshist", id.toString(), this);
    }
  }

  static load(id: string): Fundshist | null {
    return changetype<Fundshist | null>(store.get("Fundshist", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get token_id(): string {
    let value = this.get("token_id");
    return value!.toString();
  }

  set token_id(value: string) {
    this.set("token_id", Value.fromString(value));
  }

  get user_id(): string {
    let value = this.get("user_id");
    return value!.toString();
  }

  set user_id(value: string) {
    this.set("user_id", Value.fromString(value));
  }

  get type(): string {
    let value = this.get("type");
    return value!.toString();
  }

  set type(value: string) {
    this.set("type", Value.fromString(value));
  }

  get amount(): BigDecimal {
    let value = this.get("amount");
    return value!.toBigDecimal();
  }

  set amount(value: BigDecimal) {
    this.set("amount", Value.fromBigDecimal(value));
  }

  get date_time(): BigInt {
    let value = this.get("date_time");
    return value!.toBigInt();
  }

  set date_time(value: BigInt) {
    this.set("date_time", Value.fromBigInt(value));
  }
}

export class Datanft extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Datanft entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Datanft must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Datanft", id.toString(), this);
    }
  }

  static load(id: string): Datanft | null {
    return changetype<Datanft | null>(store.get("Datanft", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get total_Supply(): BigInt {
    let value = this.get("total_Supply");
    return value!.toBigInt();
  }

  set total_Supply(value: BigInt) {
    this.set("total_Supply", Value.fromBigInt(value));
  }

  get total_owners(): BigInt {
    let value = this.get("total_owners");
    return value!.toBigInt();
  }

  set total_owners(value: BigInt) {
    this.set("total_owners", Value.fromBigInt(value));
  }

  get owners(): Array<string> {
    let value = this.get("owners");
    return value!.toStringArray();
  }

  set owners(value: Array<string>) {
    this.set("owners", Value.fromStringArray(value));
  }
}

export class Owners extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Owners entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Owners must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Owners", id.toString(), this);
    }
  }

  static load(id: string): Owners | null {
    return changetype<Owners | null>(store.get("Owners", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get data_nft(): string {
    let value = this.get("data_nft");
    return value!.toString();
  }

  set data_nft(value: string) {
    this.set("data_nft", Value.fromString(value));
  }

  get owner_id(): string {
    let value = this.get("owner_id");
    return value!.toString();
  }

  set owner_id(value: string) {
    this.set("owner_id", Value.fromString(value));
  }
}

export class Serie extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Serie entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Serie must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Serie", id.toString(), this);
    }
  }

  static load(id: string): Serie | null {
    return changetype<Serie | null>(store.get("Serie", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get title(): string {
    let value = this.get("title");
    return value!.toString();
  }

  set title(value: string) {
    this.set("title", Value.fromString(value));
  }

  get description(): string | null {
    let value = this.get("description");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set description(value: string | null) {
    if (!value) {
      this.unset("description");
    } else {
      this.set("description", Value.fromString(<string>value));
    }
  }

  get media(): string {
    let value = this.get("media");
    return value!.toString();
  }

  set media(value: string) {
    this.set("media", Value.fromString(value));
  }

  get extra(): string | null {
    let value = this.get("extra");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set extra(value: string | null) {
    if (!value) {
      this.unset("extra");
    } else {
      this.set("extra", Value.fromString(<string>value));
    }
  }

  get reference(): string {
    let value = this.get("reference");
    return value!.toString();
  }

  set reference(value: string) {
    this.set("reference", Value.fromString(value));
  }

  get creator_id(): string {
    let value = this.get("creator_id");
    return value!.toString();
  }

  set creator_id(value: string) {
    this.set("creator_id", Value.fromString(value));
  }

  get price(): BigDecimal | null {
    let value = this.get("price");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigDecimal();
    }
  }

  set price(value: BigDecimal | null) {
    if (!value) {
      this.unset("price");
    } else {
      this.set("price", Value.fromBigDecimal(<BigDecimal>value));
    }
  }

  get price_near(): BigDecimal | null {
    let value = this.get("price_near");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigDecimal();
    }
  }

  set price_near(value: BigDecimal | null) {
    if (!value) {
      this.unset("price_near");
    } else {
      this.set("price_near", Value.fromBigDecimal(<BigDecimal>value));
    }
  }

  get supply(): BigInt {
    let value = this.get("supply");
    return value!.toBigInt();
  }

  set supply(value: BigInt) {
    this.set("supply", Value.fromBigInt(value));
  }

  get copies(): BigInt | null {
    let value = this.get("copies");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set copies(value: BigInt | null) {
    if (!value) {
      this.unset("copies");
    } else {
      this.set("copies", Value.fromBigInt(<BigInt>value));
    }
  }

  get fecha(): BigInt {
    let value = this.get("fecha");
    return value!.toBigInt();
  }

  set fecha(value: BigInt) {
    this.set("fecha", Value.fromBigInt(value));
  }

  get tokens(): Array<string> {
    let value = this.get("tokens");
    return value!.toStringArray();
  }

  set tokens(value: Array<string>) {
    this.set("tokens", Value.fromStringArray(value));
  }
}

export class Nft extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Nft entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Nft must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Nft", id.toString(), this);
    }
  }

  static load(id: string): Nft | null {
    return changetype<Nft | null>(store.get("Nft", id));
  }

  get id(): string {
    let value = this.get("id");
    return value!.toString();
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get serie_id(): string {
    let value = this.get("serie_id");
    return value!.toString();
  }

  set serie_id(value: string) {
    this.set("serie_id", Value.fromString(value));
  }

  get owner_id(): string {
    let value = this.get("owner_id");
    return value!.toString();
  }

  set owner_id(value: string) {
    this.set("owner_id", Value.fromString(value));
  }

  get fecha(): BigInt {
    let value = this.get("fecha");
    return value!.toBigInt();
  }

  set fecha(value: BigInt) {
    this.set("fecha", Value.fromBigInt(value));
  }

  get is_visible(): boolean {
    let value = this.get("is_visible");
    return value!.toBoolean();
  }

  set is_visible(value: boolean) {
    this.set("is_visible", Value.fromBoolean(value));
  }

  get metadata(): string {
    let value = this.get("metadata");
    return value!.toString();
  }

  set metadata(value: string) {
    this.set("metadata", Value.fromString(value));
  }
}
