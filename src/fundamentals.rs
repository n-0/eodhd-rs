use super::datetime::eodhd_serde_opt_date;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EODHDFundamentals {
    #[serde(rename = "MarketCapitalization")]
    pub market_cap: Option<f64>,
    #[serde(rename = "EBITDA")]
    pub ebitda: Option<String>,
    #[serde(rename = "PERatio")]
    pub pe_ratio: Option<f64>,
    #[serde(rename = "PEGRatio")]
    pub peg_ratio: Option<f64>,
    #[serde(rename = "WallStreetTargetPrice")]
    pub wallstreet_target_price: Option<f64>,
    #[serde(rename = "BookValue")]
    pub book_value: Option<f64>,
    #[serde(rename = "DividendShare")]
    pub dividend_share: Option<f64>,
    #[serde(rename = "DividendYield")]
    pub dividend_yield: Option<f64>,
    #[serde(rename = "EarningsShare")]
    pub earnings_share: Option<f64>,
    #[serde(rename = "EPSEstimateCurrentYear")]
    pub eps_estimate_current_year: Option<f64>,
    #[serde(rename = "EPSEstimateNextYear")]
    pub eps_estimate_next_year: Option<f64>,
    #[serde(rename = "EPSEstimateNextQuarter")]
    pub eps_estimate_next_quarter: Option<f64>,
    #[serde(rename = "EPSEstimateCurrentQuarter")]
    pub eps_estimate_current_quarter: Option<f64>,
    #[serde(rename = "MostRecentQuarter", with = "eodhd_serde_opt_date")]
    pub most_recent_quarter: Option<NaiveDate>,
    #[serde(rename = "ProfitMargin")]
    pub profit_margin: Option<f64>,
    #[serde(rename = "OperatingMarginTTM")]
    pub operating_margin_ttm: Option<f64>,
    #[serde(rename = "ReturnOnAssetsTTM")]
    pub return_on_assets_ttm: Option<f64>,
    #[serde(rename = "ReturnOnEquityTTM")]
    pub return_on_equity_ttm: Option<f64>,
    #[serde(rename = "RevenueTTM")]
    pub revenue_ttm: Option<f64>,
    #[serde(rename = "RevenuePerShareTTM")]
    pub revenue_per_share_ttm: Option<f64>,
    #[serde(rename = "QuarterlyRevenueGrowthYOY")]
    pub quarterly_revenue_growth_yoy: Option<f64>,
    #[serde(rename = "GrossProfitTTM")]
    pub gross_profit_ttm: Option<f64>,
    #[serde(rename = "DilutedEpsTTM")]
    pub diluted_eps_ttm: Option<f64>,
    #[serde(rename = "QuarterlyEarningsGrowthYOY")]
    pub quarterly_earnings_growth_yoy: Option<f64>,
}
