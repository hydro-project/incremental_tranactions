package org.dbsp.sqlCompiler.ir.type.primitive;

import org.dbsp.sqlCompiler.compiler.visitors.inner.InnerVisitor;
import org.dbsp.sqlCompiler.ir.expression.literal.DBSPLiteral;
import org.dbsp.sqlCompiler.ir.type.DBSPType;
import org.dbsp.util.Unimplemented;

public class DBSPTypeVoid extends DBSPTypeBaseType {
    public static final DBSPTypeVoid INSTANCE = new DBSPTypeVoid();

    protected DBSPTypeVoid() {
        super(null, false);
    }

    @Override
    public void accept(InnerVisitor visitor) {
        if (visitor.preorder(this).stop()) return;
        visitor.push(this);
        visitor.pop(this);
        visitor.postorder(this);
    }

    @Override
    public boolean sameType(DBSPType other) {
        return other.is(DBSPTypeVoid.class);
    }

    @Override
    public DBSPType setMayBeNull(boolean mayBeNull) {
        if (mayBeNull)
            throw new Unimplemented();
        return this;
    }

    @Override
    public String shortName() {
        return "void";
    }

    @Override
    public DBSPLiteral defaultValue() {
        throw new Unimplemented();
    }
}