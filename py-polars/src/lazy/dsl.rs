use crate::error::PyPolarsEr;
use polars::lazy::dsl;
use polars::lazy::dsl::Operator;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyInt};
use pyo3::PyNumberProtocol;

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyExpr {
    pub inner: dsl::Expr,
}

#[pyproto]
impl PyNumberProtocol for PyExpr {
    fn __add__(lhs: Self, rhs: Self) -> PyResult<PyExpr> {
        Ok(dsl::binary_expr(lhs.inner, Operator::Plus, rhs.inner).into())
    }
    fn __sub__(lhs: Self, rhs: Self) -> PyResult<PyExpr> {
        Ok(dsl::binary_expr(lhs.inner, Operator::Minus, rhs.inner).into())
    }
    fn __mul__(lhs: Self, rhs: Self) -> PyResult<PyExpr> {
        Ok(dsl::binary_expr(lhs.inner, Operator::Multiply, rhs.inner).into())
    }
    fn __truediv__(lhs: Self, rhs: Self) -> PyResult<PyExpr> {
        Ok(dsl::binary_expr(lhs.inner, Operator::Divide, rhs.inner).into())
    }
}

#[pymethods]
impl PyExpr {
    #[text_signature = "($self, other)"]
    pub fn eq(&self, other: PyExpr) -> PyExpr {
        self.clone().inner.eq(other.inner).into()
    }
    #[text_signature = "($self, other)"]
    pub fn neq(&self, other: PyExpr) -> PyExpr {
        self.clone().inner.neq(other.inner).into()
    }
    #[text_signature = "($self, other)"]
    pub fn gt(&self, other: PyExpr) -> PyExpr {
        self.clone().inner.gt(other.inner).into()
    }
    #[text_signature = "($self, other)"]
    pub fn gt_eq(&self, other: PyExpr) -> PyExpr {
        self.clone().inner.gt_eq(other.inner).into()
    }
    #[text_signature = "($self, other)"]
    pub fn lt_eq(&self, other: PyExpr) -> PyExpr {
        self.clone().inner.lt_eq(other.inner).into()
    }
    #[text_signature = "($self, other)"]
    pub fn lt(&self, other: PyExpr) -> PyExpr {
        self.clone().inner.lt(other.inner).into()
    }
    #[text_signature = "($self, name)"]
    pub fn alias(&self, name: &str) -> PyExpr {
        self.clone().inner.alias(name).into()
    }
    #[text_signature = "($self)"]
    pub fn is_not(&self) -> PyExpr {
        self.clone().inner.not().into()
    }
    #[text_signature = "($self)"]
    pub fn is_null(&self) -> PyExpr {
        self.clone().inner.is_null().into()
    }
    #[text_signature = "($self)"]
    pub fn is_not_null(&self) -> PyExpr {
        self.clone().inner.is_not_null().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_min(&self) -> PyExpr {
        self.clone().inner.agg_min().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_max(&self) -> PyExpr {
        self.clone().inner.agg_max().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_mean(&self) -> PyExpr {
        self.clone().inner.agg_mean().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_median(&self) -> PyExpr {
        self.clone().inner.agg_median().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_sum(&self) -> PyExpr {
        self.clone().inner.agg_sum().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_n_unique(&self) -> PyExpr {
        self.clone().inner.agg_n_unique().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_first(&self) -> PyExpr {
        self.clone().inner.agg_first().into()
    }
    #[text_signature = "($self)"]
    pub fn agg_last(&self) -> PyExpr {
        self.clone().inner.agg_last().into()
    }
    #[text_signature = "($self, quantile)"]
    pub fn agg_quantile(&self, quantile: f64) -> PyExpr {
        self.clone().inner.agg_quantile(quantile).into()
    }
    #[text_signature = "($self)"]
    pub fn agg_groups(&self) -> PyExpr {
        self.clone().inner.agg_groups().into()
    }

    #[text_signature = "($self, data_type)"]
    pub fn cast(&self, data_type: &str) -> PyExpr {
        // TODO! accept the DataType objects.

        let dt = str_to_arrow_type(data_type);
        let expr = self.inner.clone().cast(dt);
        expr.into()
    }
    #[text_signature = "($self, reverse)"]
    pub fn sort(&self, reverse: bool) -> PyExpr {
        self.clone().inner.sort(reverse).into()
    }
    pub fn shift(&self, periods: i32) -> PyExpr {
        self.clone().inner.shift(periods).into()
    }
    pub fn fill_none(&self, strategy: &str) -> PyResult<PyExpr> {
        let strat = match strategy {
            "backward" => FillNoneStrategy::Backward,
            "forward" => FillNoneStrategy::Forward,
            "min" => FillNoneStrategy::Min,
            "max" => FillNoneStrategy::Max,
            "mean" => FillNoneStrategy::Mean,
            s => return Err(PyPolarsEr::Other(format!("Strategy {} not supported", s)).into()),
        };
        Ok(self.clone().inner.fill_none(strat).into())
    }
    pub fn max(&self) -> PyExpr {
        self.clone().inner.max().into()
    }
    pub fn min(&self) -> PyExpr {
        self.clone().inner.min().into()
    }
    pub fn sum(&self) -> PyExpr {
        self.clone().inner.sum().into()
    }
    pub fn mean(&self) -> PyExpr {
        self.clone().inner.mean().into()
    }
    pub fn median(&self) -> PyExpr {
        self.clone().inner.median().into()
    }
    pub fn quantile(&self, quantile: f64) -> PyExpr {
        self.clone().inner.quantile(quantile).into()
    }
    pub fn str_lengths(&self) -> PyExpr {
        let function = |s: Series| {
            let ca = s.utf8()?;
            Ok(ca.str_lengths().into_series())
        };
        self.clone().inner.apply(function, None).into()
    }

    pub fn str_replace(&self, pat: String, val: String) -> PyExpr {
        let function = move |s: Series| {
            let ca = s.utf8()?;
            match ca.replace(&pat, &val) {
                Ok(ca) => Ok(ca.into_series()),
                Err(e) => Err(PolarsError::Other(format!("{:?}", e).into())),
            }
        };
        self.clone().inner.apply(function, None).into()
    }

    pub fn str_replace_all(&self, pat: String, val: String) -> PyExpr {
        let function = move |s: Series| {
            let ca = s.utf8()?;
            match ca.replace_all(&pat, &val) {
                Ok(ca) => Ok(ca.into_series()),
                Err(e) => Err(PolarsError::Other(format!("{:?}", e).into())),
            }
        };
        self.clone().inner.apply(function, None).into()
    }

    pub fn str_contains(&self, pat: String) -> PyExpr {
        let function = move |s: Series| {
            let ca = s.utf8()?;
            match ca.contains(&pat) {
                Ok(ca) => Ok(ca.into_series()),
                Err(e) => Err(PolarsError::Other(format!("{:?}", e).into())),
            }
        };
        self.clone().inner.apply(function, None).into()
    }
}

fn str_to_arrow_type(s: &str) -> ArrowDataType {
    match s {
        "u8" => ArrowDataType::UInt8,
        "u16" => ArrowDataType::UInt16,
        "u32" => ArrowDataType::UInt32,
        "u64" => ArrowDataType::UInt64,
        "i8" => ArrowDataType::Int8,
        "i16" => ArrowDataType::Int16,
        "i32" => ArrowDataType::Int32,
        "i64" => ArrowDataType::Int64,
        "f32" => ArrowDataType::Float32,
        "f64" => ArrowDataType::Float64,
        "bool" => ArrowDataType::Boolean,
        "utf8" => ArrowDataType::Utf8,
        _ => todo!(),
    }
}

impl From<dsl::Expr> for PyExpr {
    fn from(expr: dsl::Expr) -> Self {
        PyExpr { inner: expr }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct When {
    predicate: PyExpr,
}

#[pyclass]
#[derive(Clone)]
pub struct WhenThen {
    predicate: PyExpr,
    then: PyExpr,
}

#[pymethods]
impl When {
    pub fn then(&self, expr: PyExpr) -> WhenThen {
        WhenThen {
            predicate: self.predicate.clone(),
            then: expr,
        }
    }
}

#[pymethods]
impl WhenThen {
    pub fn otherwise(&self, expr: PyExpr) -> PyExpr {
        dsl::ternary_expr(
            self.predicate.inner.clone(),
            self.then.inner.clone(),
            expr.inner,
        )
        .into()
    }
}

pub fn when(predicate: PyExpr) -> When {
    When { predicate }
}

pub fn col(name: &str) -> PyExpr {
    dsl::col(name).into()
}

pub fn binary_expr(l: PyExpr, op: u8, r: PyExpr) -> PyExpr {
    let left = l.inner;
    let right = r.inner;

    let op = dsl::Operator::from(op);
    dsl::binary_expr(left, op, right).into()
}

pub fn lit(value: &PyAny) -> PyExpr {
    if let Ok(int) = value.downcast::<PyInt>() {
        let val = int.extract::<i64>().unwrap();
        dsl::lit(val).into()
    } else if let Ok(float) = value.downcast::<PyFloat>() {
        let val = float.extract::<f64>().unwrap();
        dsl::lit(val).into()
    } else {
        panic!("could not convert type")
    }
}