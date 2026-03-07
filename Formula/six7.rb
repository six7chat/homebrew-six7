class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.61"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.61"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.61/six7-0.7.61-arm64-apple-darwin.tar.gz"
      sha256 "698ae01860d5e8167da8884d0c869e9119e16656f90695604ecbceab04a7665f"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
