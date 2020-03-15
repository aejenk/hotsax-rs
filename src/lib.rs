//! A currently heavily unstable crate containing algorithms relating to data analysis
//! for anomaly detection. The API will experience numerous breaking changes for the time being until
//! the API is finalized (v1.0)

/// Implements anomaly detection algorithms, including the brute force and
/// HOT SAX algorithms as specified by Keogh's paper, found
/// [here](http://www.cse.cuhk.edu.hk/~adafu/Pub/icdm05time.pdf).
pub mod anomaly;

/// Distance algorithms between two lists of floats.
///
/// Currently only contains support for `gaussian`.
pub mod dist;

/// Dimensionality reduction techniques.
///
/// Currently only includes piecewise approximation (paa) and
/// symbolic aggregate approximation (sax)
pub mod dim_reduction;

/// Miscellaneous utility functions.
pub mod util;

pub(crate) mod trie;

#[cfg(test)]
mod test {
    use std::error::Error;
    use plotly::{Plot, Scatter};
    use crate::util::znorm;

    type Entry = f64;

    #[test]
    // Tests out the HOT SAX algorithm by running it on the same dataset as in the paper.
    fn keogh() -> Result<(), Box<dyn Error>> {
        // Parses the CSV file from the dataset.
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_path("data/TEK16.CSV")?;

        // Deserialize CSV data into a vector of floats.
        let mut data : Vec<f64> = Vec::new();
        for result in rdr.deserialize() {
            let record: Entry = result?;
            data.push(record);
        }

        // Prepare a plot
        let mut plot = Plot::new();

        // Retrieve the largest discord. This should approx. match the one found in the paper.
        // It uses the same settings: a discord size of 128 and a=3.
        // word_size was assumed to be 3.
        let discord_size = 128;
        let discord = crate::anomaly::KeoghBuilder::with(&data, discord_size)
            .find_largest_discord()
            .unwrap().1;

        // Plot the entire dataset as a blue color.
        let trace1 = Scatter::new((1..=data.len()).collect(), data.clone())
            .line(plotly::common::Line::new().color(plotly::NamedColor::Blue))
            .name("Data");

        // Plot the discord itself as a red color.
        let trace2 = Scatter::new((discord+1..discord+discord_size).collect(), data[discord..discord+128].to_vec())
            .line(plotly::common::Line::new().color(plotly::NamedColor::Red))
            .name("Discord");

        // Add traces to the plot.
        plot.add_trace(trace1);
        plot.add_trace(trace2);

        // Shows the plot to verify.
        plot.show();

        Ok(())
    }
}