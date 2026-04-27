class Pasteimg < Formula
  desc "Save the current clipboard image to Downloads as a JPEG"
  homepage "https://github.com/1337Core/pasteimg"
  url "https://github.com/1337Core/pasteimg/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "d32443fd32a26250be648c786d91afa20eb18e6e620534fe181017f0159f95a9"
  license "MIT"
  head "https://github.com/1337Core/pasteimg.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "Capture clipboard image", shell_output("#{bin}/pasteimg --help")
  end
end
