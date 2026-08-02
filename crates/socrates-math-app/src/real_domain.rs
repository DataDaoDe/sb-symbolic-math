use socrates_math_core::ExactRational;
use socrates_math_protocol::{
    CompareRealDomainsResponseDto, DiagnosticDto, ExactValueDto, MathematicalOutcomeKindDto,
    NormalizeRealDomainResponseDto, RealDomainDto, RealDomainMembershipResponseDto, RealSetDto,
};
use std::cmp::Ordering;

const DOMAIN_SCHEMA: &str = "socrates.real-domain";
const DOMAIN_VERSION: u32 = 1;
const MAX_COMPONENTS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bound {
    value: ExactRational,
    inclusive: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Segment {
    lower: Option<Bound>,
    upper: Option<Bound>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CanonicalDomain(Vec<Segment>);

pub fn normalize_response(domain: &RealDomainDto) -> NormalizeRealDomainResponseDto {
    match validate_and_normalize(domain) {
        Ok(canonical) => NormalizeRealDomainResponseDto {
            outcome: MathematicalOutcomeKindDto::Proven,
            domain: Some(to_dto(&canonical, domain)),
            diagnostics: Vec::new(),
        },
        Err(diagnostic) => unknown_normalize(diagnostic),
    }
}

pub fn intersection_response(
    left: &RealDomainDto,
    right: &RealDomainDto,
) -> NormalizeRealDomainResponseDto {
    let left_set = match validate_and_normalize(left) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_normalize(diagnostic),
    };
    let right_set = match validate_and_normalize(right) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_normalize(diagnostic),
    };
    let canonical = left_set.intersection(&right_set);
    let mut result = left.clone();
    result.provenance = socrates_math_protocol::RealDomainProvenanceDto::Restricted;
    NormalizeRealDomainResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        domain: Some(to_dto(&canonical, &result)),
        diagnostics: Vec::new(),
    }
}

pub fn comparison_response(
    left: &RealDomainDto,
    right: &RealDomainDto,
) -> CompareRealDomainsResponseDto {
    let left_set = match validate_and_normalize(left) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_compare(diagnostic),
    };
    let right_set = match validate_and_normalize(right) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_compare(diagnostic),
    };
    CompareRealDomainsResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: "domain.real.equal".to_owned(),
        equal: Some(left_set == right_set),
        left_normalized: Some(to_dto(&left_set, left)),
        right_normalized: Some(to_dto(&right_set, right)),
        diagnostics: Vec::new(),
    }
}

pub fn membership_response(
    domain: &RealDomainDto,
    value: &ExactValueDto,
) -> RealDomainMembershipResponseDto {
    let canonical = match validate_and_normalize(domain) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_membership(diagnostic),
    };
    let value = match exact_from_dto(value) {
        Ok(value) => value,
        Err(diagnostic) => return unknown_membership(diagnostic),
    };
    RealDomainMembershipResponseDto {
        outcome: MathematicalOutcomeKindDto::Proven,
        relation: "domain.real.membership".to_owned(),
        contains: Some(canonical.contains(&value)),
        normalized_domain: Some(to_dto(&canonical, domain)),
        diagnostics: Vec::new(),
    }
}

fn validate_and_normalize(domain: &RealDomainDto) -> Result<CanonicalDomain, DiagnosticDto> {
    if domain.schema != DOMAIN_SCHEMA || domain.version != DOMAIN_VERSION {
        return Err(diagnostic(
            "Domain.UnsupportedProtocol",
            "real domains require schema socrates.real-domain version 1",
        ));
    }
    CanonicalDomain::from_dto(&domain.set, 0)
}

impl CanonicalDomain {
    fn from_dto(set: &RealSetDto, depth: usize) -> Result<Self, DiagnosticDto> {
        if depth > 32 {
            return Err(diagnostic(
                "Domain.ResourceLimit",
                "domain nesting is too deep",
            ));
        }
        let segments = match set {
            RealSetDto::Empty => Vec::new(),
            RealSetDto::AllReal => vec![Segment {
                lower: None,
                upper: None,
            }],
            RealSetDto::Point { value } => {
                let value = exact_from_dto(value)?;
                vec![Segment {
                    lower: Some(Bound {
                        value: value.clone(),
                        inclusive: true,
                    }),
                    upper: Some(Bound {
                        value,
                        inclusive: true,
                    }),
                }]
            }
            RealSetDto::Interval {
                lower,
                upper,
                lower_inclusive,
                upper_inclusive,
            } => Segment::new(
                Some(Bound {
                    value: exact_from_dto(lower)?,
                    inclusive: *lower_inclusive,
                }),
                Some(Bound {
                    value: exact_from_dto(upper)?,
                    inclusive: *upper_inclusive,
                }),
            )
            .into_iter()
            .collect(),
            RealSetDto::Ray {
                direction,
                boundary,
                inclusive,
            } => {
                let bound = Bound {
                    value: exact_from_dto(boundary)?,
                    inclusive: *inclusive,
                };
                match direction.as_str() {
                    "below" => vec![Segment {
                        lower: None,
                        upper: Some(bound),
                    }],
                    "above" => vec![Segment {
                        lower: Some(bound),
                        upper: None,
                    }],
                    _ => {
                        return Err(diagnostic(
                            "Domain.InvalidRayDirection",
                            "ray direction must be below or above",
                        ));
                    }
                }
            }
            RealSetDto::Union { members } => {
                if members.len() > MAX_COMPONENTS {
                    return Err(diagnostic(
                        "Domain.ResourceLimit",
                        "domain has too many union members",
                    ));
                }
                let mut segments = Vec::new();
                for member in members {
                    segments.extend(Self::from_dto(member, depth + 1)?.0);
                }
                segments
            }
            RealSetDto::Exclude { base, points } => {
                if points.len() > MAX_COMPONENTS {
                    return Err(diagnostic(
                        "Domain.ResourceLimit",
                        "domain has too many exclusions",
                    ));
                }
                let mut domain = Self::from_dto(base, depth + 1)?;
                for point in points {
                    domain = domain.without_point(&exact_from_dto(point)?);
                }
                return Ok(domain);
            }
            RealSetDto::SetBuilder { .. } => {
                return Err(diagnostic(
                    "Domain.UnsupportedSetBuilder",
                    "set-builder real domains are outside version 1",
                ));
            }
        };
        Ok(Self::normalize(segments))
    }

    fn normalize(mut segments: Vec<Segment>) -> Self {
        segments.sort_by(compare_segments);
        let mut normalized: Vec<Segment> = Vec::new();
        for segment in segments {
            if let Some(last) = normalized.last_mut()
                && can_merge(last, &segment)
            {
                last.upper = maximum_upper(last.upper.take(), segment.upper);
                continue;
            }
            normalized.push(segment);
        }
        Self(normalized)
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut segments = Vec::new();
        for left in &self.0 {
            for right in &other.0 {
                if let Some(segment) = Segment::new(
                    maximum_lower(left.lower.clone(), right.lower.clone()),
                    minimum_upper(left.upper.clone(), right.upper.clone()),
                ) {
                    segments.push(segment);
                }
            }
        }
        Self::normalize(segments)
    }

    fn contains(&self, value: &ExactRational) -> bool {
        self.0.iter().any(|segment| segment.contains(value))
    }

    fn without_point(&self, point: &ExactRational) -> Self {
        let mut segments = Vec::new();
        for segment in &self.0 {
            if !segment.contains(point) {
                segments.push(segment.clone());
                continue;
            }
            if let Some(left) = Segment::new(
                segment.lower.clone(),
                Some(Bound {
                    value: point.clone(),
                    inclusive: false,
                }),
            ) {
                segments.push(left);
            }
            if let Some(right) = Segment::new(
                Some(Bound {
                    value: point.clone(),
                    inclusive: false,
                }),
                segment.upper.clone(),
            ) {
                segments.push(right);
            }
        }
        Self::normalize(segments)
    }
}

impl Segment {
    fn new(lower: Option<Bound>, upper: Option<Bound>) -> Option<Self> {
        if let (Some(lower), Some(upper)) = (&lower, &upper) {
            match lower.value.cmp_exact(&upper.value) {
                Ordering::Greater => return None,
                Ordering::Equal if !(lower.inclusive && upper.inclusive) => return None,
                _ => {}
            }
        }
        Some(Self { lower, upper })
    }

    fn contains(&self, value: &ExactRational) -> bool {
        let above_lower =
            self.lower
                .as_ref()
                .is_none_or(|bound| match value.cmp_exact(&bound.value) {
                    Ordering::Greater => true,
                    Ordering::Equal => bound.inclusive,
                    Ordering::Less => false,
                });
        let below_upper =
            self.upper
                .as_ref()
                .is_none_or(|bound| match value.cmp_exact(&bound.value) {
                    Ordering::Less => true,
                    Ordering::Equal => bound.inclusive,
                    Ordering::Greater => false,
                });
        above_lower && below_upper
    }
}

fn compare_segments(left: &Segment, right: &Segment) -> Ordering {
    match (&left.lower, &right.lower) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => left
            .value
            .cmp_exact(&right.value)
            .then_with(|| right.inclusive.cmp(&left.inclusive)),
    }
}

fn can_merge(left: &Segment, right: &Segment) -> bool {
    match (&left.upper, &right.lower) {
        (None, _) => true,
        (_, None) => true,
        (Some(upper), Some(lower)) => match upper.value.cmp_exact(&lower.value) {
            Ordering::Greater => true,
            Ordering::Equal => upper.inclusive || lower.inclusive,
            Ordering::Less => false,
        },
    }
}

fn maximum_lower(left: Option<Bound>, right: Option<Bound>) -> Option<Bound> {
    match (left, right) {
        (None, bound) | (bound, None) => bound,
        (Some(left), Some(right)) => match left.value.cmp_exact(&right.value) {
            Ordering::Less => Some(right),
            Ordering::Greater => Some(left),
            Ordering::Equal => Some(Bound {
                value: left.value,
                inclusive: left.inclusive && right.inclusive,
            }),
        },
    }
}

fn minimum_upper(left: Option<Bound>, right: Option<Bound>) -> Option<Bound> {
    match (left, right) {
        (None, bound) | (bound, None) => bound,
        (Some(left), Some(right)) => match left.value.cmp_exact(&right.value) {
            Ordering::Less => Some(left),
            Ordering::Greater => Some(right),
            Ordering::Equal => Some(Bound {
                value: left.value,
                inclusive: left.inclusive && right.inclusive,
            }),
        },
    }
}

fn maximum_upper(left: Option<Bound>, right: Option<Bound>) -> Option<Bound> {
    match (left, right) {
        (None, _) | (_, None) => None,
        (Some(left), Some(right)) => match left.value.cmp_exact(&right.value) {
            Ordering::Less => Some(right),
            Ordering::Greater => Some(left),
            Ordering::Equal => Some(Bound {
                value: left.value,
                inclusive: left.inclusive || right.inclusive,
            }),
        },
    }
}

fn to_dto(canonical: &CanonicalDomain, template: &RealDomainDto) -> RealDomainDto {
    let members: Vec<RealSetDto> = canonical.0.iter().map(segment_to_dto).collect();
    let set = match members.len() {
        0 => RealSetDto::Empty,
        1 => members.into_iter().next().expect("one member exists"),
        _ => RealSetDto::Union { members },
    };
    RealDomainDto {
        schema: DOMAIN_SCHEMA.to_owned(),
        version: DOMAIN_VERSION,
        set,
        provenance: template.provenance,
    }
}

fn segment_to_dto(segment: &Segment) -> RealSetDto {
    match (&segment.lower, &segment.upper) {
        (None, None) => RealSetDto::AllReal,
        (Some(lower), None) => RealSetDto::Ray {
            direction: "above".to_owned(),
            boundary: ExactValueDto::from(&lower.value),
            inclusive: lower.inclusive,
        },
        (None, Some(upper)) => RealSetDto::Ray {
            direction: "below".to_owned(),
            boundary: ExactValueDto::from(&upper.value),
            inclusive: upper.inclusive,
        },
        (Some(lower), Some(upper)) if lower.value == upper.value => RealSetDto::Point {
            value: ExactValueDto::from(&lower.value),
        },
        (Some(lower), Some(upper)) => RealSetDto::Interval {
            lower: ExactValueDto::from(&lower.value),
            upper: ExactValueDto::from(&upper.value),
            lower_inclusive: lower.inclusive,
            upper_inclusive: upper.inclusive,
        },
    }
}

fn exact_from_dto(value: &ExactValueDto) -> Result<ExactRational, DiagnosticDto> {
    let result = match value {
        ExactValueDto::Integer { value } => ExactRational::parse_integer(value),
        ExactValueDto::Rational {
            numerator,
            denominator,
        } => ExactRational::parse_fraction(numerator, denominator),
    };
    result.map_err(|error| diagnostic("Domain.InvalidExactValue", &format!("{error:?}")))
}

fn diagnostic(code: &str, message: &str) -> DiagnosticDto {
    DiagnosticDto {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}
fn unknown_normalize(diagnostic: DiagnosticDto) -> NormalizeRealDomainResponseDto {
    NormalizeRealDomainResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        domain: None,
        diagnostics: vec![diagnostic],
    }
}
fn unknown_compare(diagnostic: DiagnosticDto) -> CompareRealDomainsResponseDto {
    CompareRealDomainsResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "domain.real.equal".to_owned(),
        equal: None,
        left_normalized: None,
        right_normalized: None,
        diagnostics: vec![diagnostic],
    }
}
fn unknown_membership(diagnostic: DiagnosticDto) -> RealDomainMembershipResponseDto {
    RealDomainMembershipResponseDto {
        outcome: MathematicalOutcomeKindDto::Unknown,
        relation: "domain.real.membership".to_owned(),
        contains: None,
        normalized_domain: None,
        diagnostics: vec![diagnostic],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socrates_math_protocol::RealDomainProvenanceDto;

    fn integer(value: &str) -> ExactValueDto {
        ExactValueDto::Integer {
            value: value.to_owned(),
        }
    }

    fn domain(set: RealSetDto) -> RealDomainDto {
        RealDomainDto {
            schema: DOMAIN_SCHEMA.to_owned(),
            version: DOMAIN_VERSION,
            set,
            provenance: RealDomainProvenanceDto::Declared,
        }
    }

    #[test]
    fn equivalent_union_presentations_have_one_canonical_domain() {
        let whole = domain(RealSetDto::Interval {
            lower: integer("0"),
            upper: integer("3"),
            lower_inclusive: true,
            upper_inclusive: true,
        });
        let pieces = domain(RealSetDto::Union {
            members: vec![
                RealSetDto::Interval {
                    lower: integer("2"),
                    upper: integer("3"),
                    lower_inclusive: false,
                    upper_inclusive: true,
                },
                RealSetDto::Interval {
                    lower: integer("0"),
                    upper: integer("2"),
                    lower_inclusive: true,
                    upper_inclusive: true,
                },
            ],
        });
        let result = comparison_response(&whole, &pieces);
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(result.equal, Some(true));
    }

    #[test]
    fn contradictory_intersection_is_explicitly_empty() {
        let below = domain(RealSetDto::Ray {
            direction: "below".to_owned(),
            boundary: integer("0"),
            inclusive: false,
        });
        let above = domain(RealSetDto::Ray {
            direction: "above".to_owned(),
            boundary: integer("1"),
            inclusive: true,
        });
        let result = intersection_response(&below, &above);
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Proven);
        assert_eq!(result.domain.expect("domain").set, RealSetDto::Empty);
    }

    #[test]
    fn excluded_exact_point_survives_normalization_and_membership() {
        let punctured = domain(RealSetDto::Exclude {
            base: Box::new(RealSetDto::AllReal),
            points: vec![ExactValueDto::Rational {
                numerator: "2".to_owned(),
                denominator: "2".to_owned(),
            }],
        });
        let normalized = normalize_response(&punctured).domain.expect("domain");
        assert!(matches!(normalized.set, RealSetDto::Union { ref members } if members.len() == 2));
        let excluded = membership_response(&normalized, &integer("1"));
        let retained = membership_response(&normalized, &integer("2"));
        assert_eq!(excluded.contains, Some(false));
        assert_eq!(retained.contains, Some(true));
    }

    #[test]
    fn incompatible_domain_protocol_is_unknown() {
        let mut source = domain(RealSetDto::AllReal);
        source.version = 2;
        let result = normalize_response(&source);
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
        assert_eq!(result.domain, None);
    }

    #[test]
    fn unsupported_set_builder_claim_is_unknown() {
        let source = domain(RealSetDto::SetBuilder {
            source: "x irrational".to_owned(),
        });
        let result = normalize_response(&source);
        assert_eq!(result.outcome, MathematicalOutcomeKindDto::Unknown);
        assert_eq!(result.domain, None);
        assert_eq!(result.diagnostics[0].code, "Domain.UnsupportedSetBuilder");
    }
}
