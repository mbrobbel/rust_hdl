// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2018, Olof Kraigher olof.kraigher@gmail.com

//! Implementation of Display

use super::*;
use std::fmt::{Display, Formatter, Result};

const TAB: &'static str = "  ";

impl<T: Display> Display for WithPos<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.item)
    }
}

/// List Vec is a simple wrapper struct to print Vecs
struct DisplayVec<'a, T: Display, U: Display>(&'a Vec<T>, U);
impl<'a, T: Display, U: Display> Display for DisplayVec<'a, T, U> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let separator = &self.1;
        if let Some(item) = &self.0.iter().next() {
            write!(f, "{}", item)?;
        }
        for item in self.0.iter().skip(1) {
            write!(f, "{}{}", separator, item)?;
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
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ", &self.name)?;
        if let Some(ref signature) = &self.signature {
            write!(f, "{} ", signature)?;
        }
        write!(f, "{} ", &self.attr)?;
        if let Some(ref expr) = &self.expr {
            write!(f, "({})", expr)?;
        }
        Ok(())
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
                write!(f, "{}({})", name, DisplayVec(expressions, ','))
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
        write!(f, "{}({})", self.name, DisplayVec(&self.parameters, ','))
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
                write!(f, "{} {}", DisplayVec(choice, ','), expression)
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
                write!(f, "({})", DisplayVec(element_associations, ','))
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
            "{} {} {}",
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
                write!(f, "({})", DisplayVec(discrete_ranges, ','))?;
                if let Some(ref subtype_constraint) = subtype_constraint {
                    write!(f, "{}", subtype_constraint)?;
                }
                Ok(())
            }
            SubtypeConstraint::Record(ref element_constraints) => {
                write!(f, "({})", DisplayVec(element_constraints, ','))
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
                write!(f, "{}", DisplayVec(record_element_resolutions, ','))
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
                write!(f, "({})", DisplayVec(enumeration_literals, ','))
            }
            TypeDefinition::Integer(ref range) => write!(f, "{}", range),
            TypeDefinition::Physical(ref physical_type_declaration) => {
                write!(f, "{}", physical_type_declaration)
            }
            TypeDefinition::Array(ref array_indexes, ref subtype_indication) => write!(
                f,
                "array ({}) of {}",
                DisplayVec(array_indexes, ','),
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
            TypeDefinition::Subtype(_) => write!(f, "subtype {} is {};", self.ident, self.def),
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
            "{} {}: {}",
            self.class, self.ident, self.subtype_indication
        )?;
        if let Some(expression) = &self.expression {
            write!(f, " := {}", expression)?;
        }
        write!(f, ";")
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
            write!(f, "({})", DisplayVec(&self.parameter_list, ';'))?;
        }
        Ok(())
    }
}

impl Display for FunctionSpecification {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.pure == false {
            write!(f, "impure ")?;
        }
        write!(
            f,
            "function {} ({}) return {}",
            &self.designator,
            DisplayVec(&self.parameter_list, ';'),
            &self.return_type
        )
    }
}

impl Display for SubprogramBody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{} is", self.specification)?;
        for declaration in &self.declarations {
            writeln!(f, "{}", declaration)?;
        }
        writeln!(f, "begin")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "end")
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Signature::Function(ref selected_names, ref selected_name) => {
                write!(f, "[")?;
                for type_mark in selected_names {
                    write!(f, "{} ", type_mark)?;
                }
                write!(f, "return {}]", selected_name)
            }
            Signature::Procedure(ref selected_names) => {
                write!(f, "[")?;
                for selected_name in selected_names {
                    write!(f, " {}", selected_name)?;
                }
                write!(f, " ]")
            }
        }
    }
}

impl Display for SubprogramDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SubprogramDeclaration::Procedure(ref procedure_specification) => {
                write!(f, "{};", procedure_specification)
            }
            SubprogramDeclaration::Function(ref function_specification) => {
                write!(f, "{};", function_specification)
            }
        }
    }
}

impl Display for InterfaceFileDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "file {} : {}", self.ident, self.subtype_indication)
    }
}

impl Display for InterfaceObjectDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {}: {} {}",
            self.class, self.ident, self.mode, self.subtype_indication
        )?;
        if let Some(ref expression) = &self.expression {
            write!(f, ":= {}", expression)?;
        }
        Ok(())
    }
}

impl Display for SubprogramDefault {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SubprogramDefault::Name(ref selected_name) => write!(f, "{}", selected_name),
            SubprogramDefault::Box => write!(f, "<>"),
        }
    }
}

impl Display for InterfacePackageGenericMapAspect {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InterfacePackageGenericMapAspect::Map(ref association_elements) => {
                write!(f, "generic map ({})", DisplayVec(association_elements, ','))
            }
            InterfacePackageGenericMapAspect::Box => write!(f, "generic map (<>)"),
            InterfacePackageGenericMapAspect::Default => write!(f, "generic map ( default )"),
        }
    }
}

impl Display for InterfacePackageDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "package {} is new {} {}",
            &self.ident, &self.package_name, &self.generic_map
        )
    }
}

impl Display for InterfaceDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InterfaceDeclaration::Object(ref interface_object_declaration) => {
                write!(f, "{}", interface_object_declaration)
            }
            InterfaceDeclaration::File(ref interface_file_declaration) => {
                write!(f, "{}", interface_file_declaration)
            }
            InterfaceDeclaration::Type(ref ident) => write!(f, "{}", ident),
            InterfaceDeclaration::Subprogram(
                ref subprogram_declaration,
                ref subprogram_default,
            ) => {
                write!(f, "{}", subprogram_declaration)?;
                if let Some(ref subprogram_default) = &subprogram_default {
                    write!(f, " is {}", subprogram_default)?;
                }
                Ok(())
            }
            InterfaceDeclaration::Package(ref interface_package_declaration) => {
                write!(f, "{}", interface_package_declaration)
            }
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            _ => write!(f, "{}", format!("{:?}", self).to_lowercase()),
        }
    }
}

impl Display for PortClause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "port ({});", DisplayVec(&self.port_list, ';'))
    }
}

impl Display for ComponentDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "component {} is", &self.ident)?;
        for interface_declaration in &self.generic_list {
            writeln!(f, "{}", interface_declaration)?;
        }
        for port in &self.port_list {
            writeln!(f, "{}", port)?;
        }
        writeln!(f, "end component;")
    }
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Declaration::Object(ref object_declaration) => write!(f, "{}", object_declaration),
            Declaration::File(ref file_declaration) => write!(f, "{}", file_declaration),
            Declaration::Type(ref type_declaration) => write!(f, "{}", type_declaration),
            Declaration::Component(ref component_declaration) => {
                write!(f, "{}", component_declaration)
            }
            Declaration::Attribute(ref attribute) => write!(f, "{}", attribute),
            Declaration::Alias(ref alias_declaration) => write!(f, "{}", alias_declaration),
            Declaration::SubprogramDeclaration(ref subprogram_declaration) => {
                write!(f, "{}", subprogram_declaration)
            }
            Declaration::SubprogramBody(ref subprogram_body) => write!(f, "{}", subprogram_body),
            Declaration::Use(ref use_clause) => write!(f, "{}", use_clause),
            Declaration::Package(ref package_instantiation) => {
                write!(f, "{}", package_instantiation)
            }
            Declaration::Configuration(ref configuration_specification) => {
                write!(f, "{}", configuration_specification)
            }
        }
    }
}

impl Display for WaitStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "wait on {}", DisplayVec(&self.sensitivity_clause, ','))?;
        if let Some(ref condition_clause) = &self.condition_clause {
            write!(f, " {}", condition_clause)?;
        }
        if let Some(ref timeout_clause) = &self.timeout_clause {
            write!(f, " {}", timeout_clause)?;
        }
        write!(f, " ;")
    }
}

impl Display for AssertStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "assert {}", &self.condition)?;
        if let Some(ref report) = &self.report {
            write!(f, " report {}", report)?;
        }
        if let Some(ref severity) = &self.severity {
            write!(f, " severity {}", severity)?;
        }
        write!(f, " ;")
    }
}

impl Display for ReportStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "report {}", &self.report)?;
        if let Some(ref severity) = &self.severity {
            write!(f, " severity {}", severity)?;
        }
        write!(f, " ;")
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Target::Name(ref name) => write!(f, "{}", name),
            Target::Aggregate(ref element_associations) => {
                write!(f, "({})", DisplayVec(element_associations, ','))
            }
        }
    }
}

impl Display for WaveformElement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", &self.value)?;
        if let Some(ref after) = &self.after {
            write!(f, " after {}", after)?;
        }
        Ok(())
    }
}

impl Display for Waveform {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Waveform::Elements(ref waveform_elements) => {
                write!(f, "{}", DisplayVec(waveform_elements, ','))
            }
            Waveform::Unaffected => write!(f, "unaffected"),
        }
    }
}

impl Display for DelayMechanism {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DelayMechanism::Transport => write!(f, "transport"),
            DelayMechanism::Inertial { ref reject } => {
                if let Some(ref reject) = reject {
                    write!(f, "reject {} ", reject)?;
                }
                write!(f, "intertial")
            }
        }
    }
}

impl Display for SignalAssignment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} <=", &self.target)?;
        if let Some(delay_mechanism) = &self.delay_mechanism {
            write!(f, " {}", delay_mechanism)?;
        }
        write!(f, " {}", &self.rhs)
    }
}

impl Display for VariableAssignment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} := {};", &self.target, &self.rhs)
    }
}

impl<T: Display> Display for AssignmentRightHand<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AssignmentRightHand::Simple(ref x) => write!(f, "{}", x),
            AssignmentRightHand::Conditional(ref conditionals) => write!(f, "{}", conditionals),
            AssignmentRightHand::Selected(ref selection) => write!(f, "{}", selection),
        }
    }
}

impl<T: Display> Display for Conditional<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} when {}", &self.item, &self.condition)
    }
}

impl<T: Display> Display for Conditionals<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", DisplayVec(&self.conditionals, " else"))?;
        if let Some(ref else_item) = &self.else_item {
            write!(f, " else {}", else_item)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for Alternative<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "when {} => {}",
            DisplayVec(&self.choices, '|'),
            &self.item
        )
    }
}

impl<T: Display> Display for Selection<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "case {} is", &self.expression)?;
        for alternative in &self.alternatives {
            write!(f, "{}", alternative)?;
        }
        writeln!(f, "end case;")
    }
}

impl Display for IterationScheme {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            IterationScheme::While(ref expression) => write!(f, "while {}", expression),
            IterationScheme::For(ref ident, ref discrete_range) => {
                write!(f, "for {} in {}", ident, discrete_range)
            }
        }
    }
}

impl Display for LoopStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref iteration_scheme) = &self.iteration_scheme {
            write!(f, "{} ", iteration_scheme)?;
        }
        writeln!(f, "loop")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "end loop;")
    }
}

impl Display for NextStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "next")?;
        if let Some(ref loop_label) = &self.loop_label {
            write!(f, " {}", loop_label)?;
        }
        if let Some(ref condition) = &self.condition {
            write!(f, " when {}", condition)?;
        }
        Ok(())
    }
}

impl Display for ExitStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "exit")?;
        if let Some(ref loop_label) = &self.loop_label {
            write!(f, " {}", loop_label)?;
        }
        if let Some(ref condition) = &self.condition {
            write!(f, " when {}", condition)?;
        }
        Ok(())
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "return")?;
        if let Some(ref expression) = &self.expression {
            write!(f, " {}", expression)?;
        }
        write!(f, ";")
    }
}

impl Display for SequentialStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SequentialStatement::Wait(ref wait_statement) => write!(f, "{}", wait_statement),
            SequentialStatement::Assert(ref assert_statement) => write!(f, "{}", assert_statement),
            SequentialStatement::Report(ref report_statement) => write!(f, "{}", report_statement),
            SequentialStatement::VariableAssignment(ref variable_assignment) => {
                write!(f, "{}", variable_assignment)
            }
            SequentialStatement::SignalAssignment(ref signal_assignment) => {
                write!(f, "{}", signal_assignment)
            }
            SequentialStatement::ProcedureCall(ref function_call) => write!(f, "{}", function_call),
            SequentialStatement::If(ref if_statement) => write!(f, "if-statement"),
            // SequentialStatement::If(ref if_statement) => write!(f, "{}", if_statement),
            SequentialStatement::Case(ref case_statement) => write!(f, "case-statment"),
            // SequentialStatement::Case(ref case_statement) => write!(f, "{}", case_statement),
            SequentialStatement::Loop(ref loop_statement) => write!(f, "{}", loop_statement),
            SequentialStatement::Next(ref next_statement) => write!(f, "{}", next_statement),
            SequentialStatement::Exit(ref exit_statement) => write!(f, "{}", exit_statement),
            SequentialStatement::Return(ref return_statement) => write!(f, "{}", return_statement),
            SequentialStatement::Null => write!(f, "null;"),
        }
    }
}

impl Display for LabeledSequentialStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref label) = &self.label {
            write!(f, "{}: ", label)?;
        }
        write!(f, "{}", &self.statement)
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "block ")?;
        if let Some(ref guard_condition) = &self.guard_condition {
            write!(f, "({}) ", guard_condition)?;
        }
        write!(f, "is")?;
        for declaration in &self.decl {
            writeln!(f, "{}", declaration)?;
        }
        writeln!(f, "begin")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "end block;")
    }
}

impl Display for SensitivityList {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SensitivityList::Names(ref names) => write!(f, "{}", DisplayVec(names, ',')),
            SensitivityList::All => write!(f, "all"),
        }
    }
}

impl Display for ProcessStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.postponed {
            write!(f, "postponed ")?;
        }
        write!(f, "process ")?;
        if let Some(ref sensitivity_list) = &self.sensitivity_list {
            write!(f, "({}) ", sensitivity_list)?;
        }
        writeln!(f, "is")?;
        for declaration in &self.decl {
            writeln!(f, "{}", declaration)?;
        }
        writeln!(f, "begin")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "end ")?;
        if self.postponed {
            write!(f, "postponed ")?;
        }
        writeln!(f, "process;")
    }
}

impl Display for ConcurrentProcedureCall {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.postponed {
            write!(f, "postponed ")?;
        }
        write!(f, "{};", &self.call)
    }
}

impl Display for ConcurrentAssertStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.postponed {
            write!(f, "postponed ")?;
        }
        write!(f, "{};", &self.statement)
    }
}

impl Display for ConcurrentSignalAssignment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.postponed {
            write!(f, "postponed ")?;
        }
        write!(f, "{} <= ", &self.target)?;
        if self.guarded {
            write!(f, "guarded ")?;
        }
        if let Some(ref delay_mechanism) = &self.delay_mechanism {
            write!(f, "{} ", delay_mechanism)?;
        }
        write!(f, "{};", &self.rhs)
    }
}

impl Display for InstantiatedUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InstantiatedUnit::Component(ref selected_name) => {
                write!(f, "component {}", selected_name)
            }
            InstantiatedUnit::Entity(ref selected_name, ref ident) => {
                write!(f, "entity {}", selected_name)?;
                if let Some(ref ident) = ident {
                    write!(f, "({})", ident)?;
                }
                Ok(())
            }
            InstantiatedUnit::Configuration(ref selected_name) => {
                write!(f, "configuration {}", selected_name)
            }
        }
    }
}

impl Display for InstantiationStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "{}", &self.unit)?;
        if !&self.generic_map.is_empty() {
            writeln!(f, "generic map ({})", DisplayVec(&self.generic_map, ','))?;
        }
        if !&self.port_map.is_empty() {
            writeln!(f, "port map ({})", DisplayVec(&self.port_map, ','))?;
        }
        write!(f, ";")
    }
}

impl Display for GenerateBody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref alternative_label) = &self.alternative_label {
            write!(f, "{}: ", alternative_label)?;
        }
        if let Some(ref declarations) = &self.decl {
            for declaration in declarations {
                writeln!(f, "{}", declaration)?;
            }
        }
        writeln!(f, "begin")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "end;")
    }
}

impl Display for ForGenerateStatement {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        unimplemented!()
    }
}

// TODO: find solution for this
// impl Display for IfGenerateStatement {
//     fn fmt(&self, _f: &mut Formatter) -> Result {
//         unimplemented!()
//     }
// }
//
// impl Display for CaseGenerateStatement {
//     fn fmt(&self, _f: &mut Formatter) -> Result {
//         unimplemented!()
//     }
// }

impl Display for ConcurrentStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ConcurrentStatement::ProcedureCall(ref concurrent_procedure_call) => {
                write!(f, "{}", concurrent_procedure_call)
            }
            ConcurrentStatement::Block(ref block_statement) => write!(f, "{}", block_statement),
            ConcurrentStatement::Process(ref process_statement) => {
                write!(f, "{}", process_statement)
            }
            ConcurrentStatement::Assert(ref concurrent_assert_statement) => {
                write!(f, "{}", concurrent_assert_statement)
            }
            ConcurrentStatement::Assignment(ref concurrent_signal_assignment) => {
                write!(f, "{}", concurrent_signal_assignment)
            }
            ConcurrentStatement::Instance(ref instantiation_statement) => {
                write!(f, "{}", instantiation_statement)
            }
            ConcurrentStatement::ForGenerate(ref for_generate_statement) => {
                write!(f, "{}", for_generate_statement)
            }
            ConcurrentStatement::IfGenerate(ref if_generate_statement) => {
                write!(f, "{}", if_generate_statement)
            }
            ConcurrentStatement::CaseGenerate(ref case_generate_statement) => {
                write!(f, "{}", case_generate_statement)
            }
        }
    }
}

impl Display for LabeledConcurrentStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref label) = &self.label {
            write!(f, "{}: ", label)?;
        }
        write!(f, "{}", &self.statement)
    }
}

impl Display for LibraryClause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "library {};", DisplayVec(&self.name_list, ','))
    }
}

impl Display for UseClause {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "use {};", DisplayVec(&self.name_list, ','))
    }
}

impl Display for ContextReference {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "context {};", DisplayVec(&self.name_list, ','))
    }
}

impl Display for ContextItem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ContextItem::Use(ref use_clause) => write!(f, "{}", use_clause),
            ContextItem::Library(ref library_clause) => write!(f, "{}", library_clause),
            ContextItem::Context(ref context_reference) => write!(f, "{}", context_reference),
        }
    }
}

impl Display for ContextDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "context {} is", &self.ident)?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        writeln!(f, "end context")
    }
}

impl Display for PackageInstantiation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "package {} is new {}", &self.ident, &self.package_name)?;
        if let Some(ref generic_map) = &self.generic_map {
            write!(f, " generic map ({})", DisplayVec(generic_map, ','))?;
        }
        write!(f, ";")
    }
}

impl Display for InstantiationList {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InstantiationList::Labels(ref idents) => write!(f, "{}", DisplayVec(idents, ',',)),
            InstantiationList::Others => write!(f, "others"),
            InstantiationList::All => write!(f, "all"),
        }
    }
}

impl Display for EntityAspect {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            EntityAspect::Entity(ref selected_name, ref ident) => {
                write!(f, "use entity {}", selected_name)?;
                if let Some(ref ident) = ident {
                    write!(f, " ({})", ident)?;
                }
                Ok(())
            }
            EntityAspect::Configuration(ref selected_name) => {
                write!(f, "configuration {}", selected_name)
            }
            EntityAspect::Open => write!(f, "open"),
        }
    }
}

impl Display for BindingIndication {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref entity_aspect) = &self.entity_aspect {
            writeln!(f, "{}", entity_aspect)?;
        }
        if let Some(ref association_elements) = &self.generic_map {
            writeln!(f, "generic map ({})", DisplayVec(association_elements, ','))?;
        }
        if let Some(ref association_elements) = &self.port_map {
            writeln!(f, "port map ({})", DisplayVec(association_elements, ','))?;
        }
        Ok(())
    }
}

impl Display for ComponentSpecification {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", &self.instantiation_list, &self.component_name)
    }
}

impl Display for VUnitBindingIndication {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "use vunit {}", DisplayVec(&self.vunit_list, ','))
    }
}

impl Display for ConfigurationSpecification {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "for {} {};", &self.spec, &self.bind_ind)?;
        for vunit_bind_indication in &self.vunit_bind_inds {
            writeln!(f, "{};", vunit_bind_indication)?;
        }
        writeln!(f, "end for;")
    }
}

impl Display for ConfigurationDeclarativeItem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ConfigurationDeclarativeItem::Use(ref use_clause) => write!(f, "{}", use_clause),
        }
    }
}

impl Display for ComponentConfiguration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "for {} ", &self.spec)?;
        if let Some(ref bind_ind) = &self.bind_ind {
            write!(f, "{} ", bind_ind)?;
        }
        for vunit_bind_indication in &self.vunit_bind_inds {
            writeln!(f, "{};", vunit_bind_indication)?;
        }
        if let Some(ref block_config) = &self.block_config {
            writeln!(f, "{}", block_config)?;
        }
        writeln!(f, "end for;")
    }
}

impl Display for ConfigurationItem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ConfigurationItem::Block(ref block_configuration) => {
                write!(f, "{}", block_configuration)
            }
            ConfigurationItem::Component(ref component_configuration) => {
                write!(f, "{}", component_configuration)
            }
        }
    }
}

impl Display for BlockConfiguration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "for {}", &self.block_spec)?;
        for use_clause in &self.use_clauses {
            writeln!(f, "{}", use_clause)?;
        }
        for configuration_item in &self.items {
            writeln!(f, "{}", configuration_item)?;
        }
        writeln!(f, "end for;")
    }
}

impl Display for ConfigurationDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "configuration {} of {} is",
            &self.ident, &self.entity_name
        )?;
        for decl in &self.decl {
            writeln!(f, "{}", decl)?;
        }
        for vunit_bind_indication in &self.vunit_bind_inds {
            writeln!(f, "{};", vunit_bind_indication)?;
        }
        writeln!(f, "{}", &self.block_config)
    }
}

impl Display for EntityDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "entity {} is", &self.ident)?;
        if let Some(ref generic_clause) = &self.generic_clause {
            writeln!(f, "generic ({})", DisplayVec(generic_clause, ','))?;
        }
        if let Some(ref port_clause) = &self.port_clause {
            writeln!(f, "port ({})", DisplayVec(port_clause, ','))?;
        }
        for decl in &self.decl {
            writeln!(f, "{}", decl)?;
        }
        writeln!(f, "begin")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "end entity;")
    }
}

impl Display for ArchitectureBody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "architecture {} of {} is",
            &self.ident, &self.entity_name
        )?;
        writeln!(f, "begin")?;
        for decl in &self.decl {
            writeln!(f, "{}", decl)?;
        }
        writeln!(f, "end architecture;")
    }
}

impl Display for PackageDeclaration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "package {} is", &self.ident)?;
        if let Some(ref generic_clause) = &self.generic_clause {
            writeln!(f, "generic ({})", DisplayVec(generic_clause, ','))?;
        }
        for decl in &self.decl {
            writeln!(f, "{}{}", TAB, decl)?;
        }
        writeln!(f, "end package;")
    }
}

impl Display for PackageBody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "package body {} is", &self.ident)?;
        for decl in &self.decl {
            writeln!(f, "{}{}", TAB, decl)?;
        }
        writeln!(f, "end package body;")
    }
}

impl Display for PrimaryUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            PrimaryUnit::EntityDeclaration(ref design_unit) => write!(f, "{}", design_unit),
            PrimaryUnit::Configuration(ref design_unit) => write!(f, "{}", design_unit),
            PrimaryUnit::PackageDeclaration(ref design_unit) => write!(f, "{}", design_unit),
            PrimaryUnit::PackageInstance(ref design_unit) => write!(f, "{}", design_unit),
            PrimaryUnit::ContextDeclaration(ref context_declaration) => {
                write!(f, "{}", context_declaration)
            }
        }
    }
}

impl Display for SecondaryUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            SecondaryUnit::Architecture(ref design_unit) => write!(f, "{}", design_unit),
            SecondaryUnit::PackageBody(ref design_unit) => write!(f, "{}", design_unit),
        }
    }
}

impl<T: Display> Display for DesignUnit<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for context_clause in &self.context_clause {
            writeln!(f, "{}", context_clause)?;
        }
        if !&self.context_clause.is_empty() {
            writeln!(f)?;
        }
        write!(f, "{}", &self.unit)
    }
}

impl Display for AnyDesignUnit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AnyDesignUnit::Primary(ref primary_unit) => write!(f, "{}", primary_unit),
            AnyDesignUnit::Secondary(ref secondary_unit) => write!(f, "{}", secondary_unit),
        }
    }
}

impl Display for DesignFile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for unit in &self.design_units {
            writeln!(f, "{}", unit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::latin_1::Latin1String;

    #[test]
    fn display_vec() {
        let a = vec![1, 2, 3];
        assert_eq!(format!("{}", DisplayVec(&a, ',')), "1,2,3".to_owned());
        let b = vec!["a", "b", "c"];
        assert_eq!(format!("{};", DisplayVec(&b, "; ")), "a; b; c;".to_owned());
    }

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
