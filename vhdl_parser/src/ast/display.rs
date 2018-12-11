// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2018, Olof Kraigher olof.kraigher@gmail.com

//! Implementation of Display

use super::*;
use std::fmt::{Display, Formatter, Result};

impl<T: Display> Display for WithPos<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.item)
    }
}

/// List Vec is a simple wrapper struct to print Vecs
struct DisplayVec<'a, T: Display>(&'a Vec<T>);
impl<'a, T: Display> Display for DisplayVec<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(item) = &self.0.first() {
            write!(f, "{}", item)?;
        }
        for item in &self.0.iter().next() {
            write!(f, ",{}", item)?;
        }
        Ok(())
    }
}

impl Display for BaseSpecifier {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Unary::Minus => write!(f, "-"),
            Unary::Plus => write!(f, "+"),
            Unary::QueQue => write!(f, "??"),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Binary::EQ => write!(f, "="),
            Binary::NE => write!(f, "/="),
            Binary::LT => write!(f, "<"),
            Binary::LTE => write!(f, "<="),
            Binary::GT => write!(f, ">"),
            Binary::GTE => write!(f, ">="),
            Binary::QueEQ => write!(f, "?="),
            Binary::QueNE => write!(f, "?/="),
            Binary::QueLT => write!(f, "?<"),
            Binary::QueLTE => write!(f, "?<="),
            Binary::QueGT => write!(f, "?>"),
            Binary::QueGTE => write!(f, "?>="),
            Binary::Plus => write!(f, "+"),
            Binary::Minus => write!(f, "-"),
            Binary::Concat => write!(f, "&"),
            Binary::Times => write!(f, "*"),
            Binary::Div => write!(f, "/"),
            Binary::Pow => write!(f, "**"),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}

impl Display for AttributeName {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ExternalObjectClass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for ExternalPath {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ExternalPath::Package(ref name) => write!(f, "{}", format!("{}", name)),
            ExternalPath::Absolute(ref name) => write!(f, "{}", format!("{}", name)),
            ExternalPath::Relative(ref name) => write!(f, "{}", format!(".{}", name)),
        }
    }
}

impl Display for ExternalName {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} : {}", self.class, self.path, self.subtype)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Name::Designator(ref designator) => write!(f, "{}", designator),
            Name::Selected(ref name, ref designator) => write!(f, "{}.{}", name, designator),
            Name::SelectedAll(ref name) => write!(f, "{}.all", name),
            Name::Indexed(ref name, ref expressions) => {
                write!(f, "{}({})", name, DisplayVec(expressions))
            }
            Name::Slice(ref name, ref discrete_range) => write!(f, "{}({})", name, discrete_range),
            Name::Attribute(ref attribute_name) => write!(f, "{}", attribute_name),
            Name::FunctionCall(ref function_call) => write!(f, "{}", function_call),
            Name::External(ref external_name) => write!(f, "{}", external_name),
        }
    }
}

impl Display for SelectedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SelectedName::Selected(ref prefix, ref des) => write!(f, "{}.{}", prefix, des),
            SelectedName::Designator(ref des) => write!(f, "{}", des),
        }
    }
}

impl Display for FunctionCall {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}({})", self.name, DisplayVec(&self.parameters))
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Choice::Expression(ref expression) => write!(f, "{}", expression),
            Choice::DiscreteRange(ref discrete_range) => write!(f, "{}", discrete_range),
            Choice::Others => write!(f, "others"),
        }
    }
}

impl Display for ElementAssociation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ElementAssociation::Positional(ref expression) => write!(f, "{}", expression),
            ElementAssociation::Named(ref choice, ref expression) => {
                // TODO: implement other display vec with =>
                write!(f, "{} {}", DisplayVec(choice), expression)
            }
        }
    }
}

impl Display for ActualPart {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ActualPart::Expression(ref expression) => write!(f, "{}", expression),
            ActualPart::Open => write!(f, "open"),
        }
    }
}

impl Display for AssociationElement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref name) = self.formal {
            write!(f, "{} => ", name)?;
        }
        write!(f, "{}", self.actual)
    }
}

impl Display for AbstractLiteral {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO: make this smart
        // abstract_literal ::=  decimal_literal | based_literal
        // decimal_literal ::=  integer [ . integer ] [ exponent ]
        // integer ::=  digit  { [ underline ] digit }
        // exponent ::=  E [ + ] integer | E – integer
        // based_literal ::= base # based_integer [ . based_integer ] # [ exponent ]
        // base ::=  integer
        // based_integer ::= extended_digit { [ underline ] extended_digit }
        // extended_digit ::=  digit | letter
        match self {
            AbstractLiteral::Integer(ref x) => write!(f, "{}", x),
            AbstractLiteral::Real(ref x) => write!(f, "{}", x),
        }
    }
}

impl Display for BitString {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(length) = self.length {
            write!(f, r#"{}{}"{}""#, length, self.base, self.value)
        } else {
            write!(f, r#"{}"{}""#, self.base, self.value)
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Literal::String(ref latin_1) => write!(f, r#"{}"#, latin_1),
            Literal::BitString(ref bitstring) => write!(f, "{}", bitstring),
            Literal::Character(x) => write!(f, "'{}'", char::from(*x)),
            Literal::AbstractLiteral(ref abstract_literal) => write!(f, "{}", abstract_literal),
            Literal::Physical(ref abstract_literal, ref symbol) => {
                write!(f, "{} {}", abstract_literal, symbol)
            }
            Literal::Null => write!(f, "null"),
        }
    }
}

impl Display for Allocator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Allocator::Qualified(ref qualified_expression) => write!(f, "{}", qualified_expression),
            Allocator::Subtype(ref subtype_indication) => write!(f, "{}", subtype_indication),
        }
    }
}

impl Display for QualifiedExpression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}'({})'", self.name, self.expr)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expression::Binary(ref operator, ref left, ref right) => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expression::Unary(ref operator, ref operand) => write!(f, "({} {})", operator, operand),
            Expression::Aggregate(ref element_associations) => {
                write!(f, "({})", DisplayVec(element_associations))
            }
            Expression::Qualified(ref qualified_expression) => {
                write!(f, "{}", qualified_expression)
            }
            Expression::Name(ref name) => write!(f, "{}", name),
            Expression::Literal(ref literal) => write!(f, "{}", literal),
            Expression::New(ref allocator) => write!(f, "new {}", allocator),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Direction::Ascending => write!(f, "to"),
            Direction::Descending => write!(f, "downto"),
        }
    }
}

impl Display for DiscreteRange {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DiscreteRange::Discrete(ref selected_name, ref range) => {
                write!(f, "{}", selected_name)?;
                if let Some(ref range) = range {
                    write!(f, "{}", range)?;
                }
                Ok(())
            }
            DiscreteRange::Range(ref range) => write!(f, "{}", range),
        }
    }
}

impl Display for RangeConstraint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "range {} {} {}",
            self.left_expr, self.direction, self.right_expr
        )
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Range::Range(ref range_constraint) => write!(f, "{}", range_constraint),
            Range::Attribute(ref attribute_name) => write!(f, "{}", attribute_name),
        }
    }
}

impl Display for ElementConstraint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.ident, self.constraint)
    }
}

impl Display for SubtypeConstraint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SubtypeConstraint::Range(ref range) => write!(f, "{}", range),
            SubtypeConstraint::Array(ref discrete_ranges, ref subtype_constraint) => {
                write!(f, "({})", DisplayVec(discrete_ranges))?;
                if let Some(ref subtype_constraint) = subtype_constraint {
                    write!(f, "{}", subtype_constraint)?;
                }
                Ok(())
            }
            SubtypeConstraint::Record(ref element_constraints) => {
                write!(f, "({})", DisplayVec(element_constraints))
            }
        }
    }
}

impl Display for RecordElementResolution {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.ident, self.resolution)
    }
}

impl Display for ResolutionIndication {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ResolutionIndication::FunctionName(ref selected_name) => write!(f, "{}", selected_name),
            ResolutionIndication::ArrayElement(ref selected_name) => write!(f, "{}", selected_name),
            ResolutionIndication::Record(ref record_element_resolutions) => {
                write!(f, "{}", DisplayVec(record_element_resolutions))
            }
            ResolutionIndication::Unresolved => Ok(()),
        }
    }
}

impl Display for SubtypeIndication {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {}", self.resolution, self.type_mark)?;
        if let Some(ref constraint) = self.constraint {
            write!(f, "{}", constraint)?;
        }
        Ok(())
    }
}

impl Display for ArrayIndex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ArrayIndex::IndexSubtypeDefintion(ref selected_name) => {
                write!(f, "{} range <>", selected_name)
            }
            ArrayIndex::Discrete(ref discrete_range) => write!(f, "{}", discrete_range),
        }
    }
}

impl Display for ElementDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} : {};", self.ident, self.subtype)
    }
}

impl Display for ProtectedTypeDeclarativeItem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ProtectedTypeDeclarativeItem::Subprogram(ref subprogram_declaration) => {
                write!(f, "{}", subprogram_declaration)
            }
        }
    }
}

impl Display for Designator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Designator::Identifier(ref sym) => write!(f, "{}", sym),
            Designator::OperatorSymbol(ref latin1) => write!(f, r#""{}""#, latin1),
            Designator::Character(byte) => write!(f, "'{}'", char::from(*byte)),
        }
    }
}

impl Display for AliasDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "alias {}", self.designator)?;
        if let Some(ref subtype_indication) = self.subtype_indication {
            write!(f, " : {}", subtype_indication)?;
        }
        write!(f, " is {}", self.name)?;
        if let Some(ref signature) = self.signature {
            write!(f, " {} ", signature)?;
        }
        write!(f, ";")
    }
}

impl Display for AttributeDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "attribute {} : {};", self.ident, self.type_mark)
    }
}

impl Display for EntityTag {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.designator)?;
        if let Some(ref signature) = self.signature {
            write!(f, " {}", signature)?;
        }
        Ok(())
    }
}

impl Display for EntityName {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            EntityName::Name(ref entity_tag) => write!(f, "{}", entity_tag),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}

impl Display for EntityClass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for AttributeSpecification {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "attribute {} of {} : {} is {};",
            self.ident, self.entity_name, self.entity_class, self.expr
        )
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Attribute::Specification(ref attribute_specification) => {
                write!(f, "{}", attribute_specification)
            }
            Attribute::Declaration(ref attribute_declaration) => {
                write!(f, "{}", attribute_declaration)
            }
        }
    }
}

impl Display for ProtectedTypeDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "protected")?;
        for item in &self.items {
            write!(f, "{}", item)?;
        }
        writeln!(f, "end protected")
    }
}

impl Display for ProtectedTypeBody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "protected body")?;
        for declaration in &self.decl {
            write!(f, "{}", declaration)?;
        }
        writeln!(f, "end protected body")
    }
}

impl Display for PhysicalTypeDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.range)?;
        writeln!(f, "units")?;
        writeln!(f, "{};", self.primary_unit)?;
        for (ident, literal) in &self.secondary_units {
            writeln!(f, "{} = {};", ident, literal)?;
        }
        writeln!(f, "end units")
    }
}

impl Display for EnumerationLiteral {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            EnumerationLiteral::Identifier(ref symbol) => write!(f, "{}", symbol),
            EnumerationLiteral::Character(ref byte) => write!(f, "'{}'", char::from(*byte)),
        }
    }
}

impl Display for TypeDefinition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TypeDefinition::Enumeration(ref enumeration_literals) => {
                write!(f, "({})", DisplayVec(enumeration_literals))
            }
            TypeDefinition::Integer(ref range) => write!(f, "{}", range),
            TypeDefinition::Physical(ref physical_type_declaration) => {
                write!(f, "{}", physical_type_declaration)
            }
            TypeDefinition::Array(ref array_indexes, ref subtype_indication) => write!(
                f,
                "array ({}) of {}",
                DisplayVec(array_indexes),
                subtype_indication
            ),
            TypeDefinition::Record(ref element_declarations) => {
                writeln!(f, "record")?;
                for element_declaration in element_declarations {
                    write!(f, "{}", element_declaration)?;
                }
                writeln!(f, "end record")
            }
            TypeDefinition::Access(ref subtype_indication) => {
                write!(f, "access {}", subtype_indication)
            }
            TypeDefinition::Incomplete => Ok(()),
            TypeDefinition::File(ref selected_name) => write!(f, "file of {}", selected_name),
            TypeDefinition::Protected(ref protected_type_declaration) => {
                write!(f, "{}", protected_type_declaration)
            }
            TypeDefinition::ProtectedBody(ref protected_type_body) => {
                write!(f, "{}", protected_type_body)
            }
            TypeDefinition::Subtype(ref subtype_indication) => write!(f, "{}", subtype_indication),
        }
    }
}

impl Display for TypeDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.def {
            TypeDefinition::Incomplete => write!(f, "type {};", self.ident),
            _ => write!(f, "type {} is {};", self.ident, self.def),
        }
    }
}

impl Display for ObjectClass {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ObjectClass::SharedVariable => write!(f, "shared variable"),
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}

impl Display for ObjectDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {} : {}",
            self.class, self.ident, self.subtype_indication
        )?;
        if let Some(expression) = &self.expression {
            write!(f, " := {}", expression)?;
        }
        Ok(())
    }
}

impl Display for FileDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "file {} : {}", self.ident, self.subtype_indication)?;
        if let Some(ref expression) = self.open_info {
            write!(f, " open {}", expression)?;
        }
        if let Some(ref expression) = self.file_name {
            write!(f, " {}", expression)?;
        }
        Ok(())
    }
}

impl Display for SubprogramDesignator {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SubprogramDesignator::Identifier(ref symbol) => write!(f, "{}", symbol),
            SubprogramDesignator::OperatorSymbol(ref latin1) => write!(f, "{}", latin1),
        }
    }
}

impl Display for ProcedureSpecification {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "procedure {}", self.designator)?;
        if !self.parameter_list.is_empty() {
            write!(f, "({})", DisplayVec(&self.parameter_list))?;
        }
        Ok(())
    }
}

impl Display for FunctionSpecification {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SubprogramBody {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for Signature {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SubprogramDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InterfaceObjectDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SubprogramDefault {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InterfacePackageGenericMapAspect {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InterfacePackageDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InterfaceDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for Mode {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for PortClause {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ComponentDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for Declaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for WaitStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for AssertStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ReportStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for Target {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for WaveformElement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for Waveform {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for DelayMechanism {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SignalAssignment {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for VariableAssignment {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl<T> Display for AssignmentRightHand<T> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl<T> Display for Conditional<T> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl<T> Display for Conditionals<T> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl<T> Display for Alternative<T> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl<T> Display for Selection<T> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for IterationScheme {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for LoopStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for NextStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ExitStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SequentialStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for LabeledSequentialStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for BlockStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SensitivityList {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ProcessStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConcurrentProcedureCall {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConcurrentAssertStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConcurrentSignalAssignment {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InstantiatedUnit {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InstantiationStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for GenerateBody {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ForGenerateStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConcurrentStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for LabeledConcurrentStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for LibraryClause {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for UseClause {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ContextReference {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ContextItem {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ContextDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for PackageInstantiation {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for InstantiationList {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for EntityAspect {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for BindingIndication {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ComponentSpecification {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for VUnitBindingIndication {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConfigurationSpecification {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConfigurationDeclarativeItem {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ComponentConfiguration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConfigurationItem {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for BlockConfiguration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ConfigurationDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for EntityDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for ArchitectureBody {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for PackageDeclaration {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for PackageBody {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for PrimaryUnit {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for SecondaryUnit {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl<T> Display for DesignUnit<T> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for AnyDesignUnit {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

impl Display for DesignFile {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::latin_1::Latin1String;

    #[test]
    fn formatter_debug_tests() {
        assert_eq!(
            format!(
                "{}",
                BitString {
                    length: None,
                    base: BaseSpecifier::SX,
                    value: Latin1String::from_utf8("0FF").unwrap()
                }
            ),
            r#"SX"0FF""#.to_owned()
        );
        assert_eq!(
            format!(
                "{}",
                BitString {
                    length: Some(12u32),
                    base: BaseSpecifier::UB,
                    value: Latin1String::from_utf8("X1").unwrap()
                }
            ),
            r#"12UB"X1""#.to_owned()
        );
    }
}
